use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use tauri::Emitter;
use tokio::task;
use rubato::{Resampler, SincFixedIn, SincInterpolationType, SincInterpolationParameters, WindowFunction};

pub struct AudioSystem {
    pub stream: Option<cpal::Stream>,
    pub pcm_buffer: Arc<Mutex<Vec<f32>>>,
    pub is_recording: Arc<Mutex<bool>>,
    pub tx: mpsc::Sender<Vec<f32>>,
    pub rx: Arc<Mutex<mpsc::Receiver<Vec<f32>>>>,
}

// SAFETY: cpal::Stream is not Send on all platforms (like macOS), 
// but we synchronize all access via Mutex in OnceLock.
unsafe impl Send for AudioSystem {}

impl AudioSystem {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            stream: None,
            pcm_buffer: Arc::new(Mutex::new(Vec::new())),
            is_recording: Arc::new(Mutex::new(false)),
            tx,
            rx: Arc::new(Mutex::new(rx)),
        }
    }

    pub fn start_capture(&mut self, app: tauri::AppHandle) -> Result<(), String> {
        let host = cpal::default_host();
        let device = host.default_input_device().ok_or("No input device available")?;
        
        let config = device.default_input_config().map_err(|e| e.to_string())?;
        let sample_rate = config.sample_rate().0;
        let channels = config.channels();

        let tx = self.tx.clone();
        *self.is_recording.lock().unwrap() = true;

        let err_fn = move |err| {
            log::error!("an error occurred on stream: {}", err);
        };

        // We capture raw f32 data.
        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => device.build_input_stream(
                &config.into(),
                move |data: &[f32], _: &_| {
                    if let Some(mono_data) = Self::to_mono(data, channels) {
                        let _ = tx.send(mono_data);
                    }
                },
                err_fn,
                None, // Provide None instead of Timeout
            ).map_err(|e| e.to_string())?,
            _ => return Err("Unsupported sample format. Only f32 is supported for now.".into()),
        };

        stream.play().map_err(|e| e.to_string())?;
        self.stream = Some(stream);

        // Spawn Tokio re-sampler task
        let pcm_buffer_cloned = self.pcm_buffer.clone();
        let is_recording_cloned = self.is_recording.clone();
        let rx_cloned = self.rx.clone();

        task::spawn_blocking(move || {
            let mut resampler = SincFixedIn::<f32>::new(
                16000_f64 / sample_rate as f64,
                2.0,
                SincInterpolationParameters {
                    sinc_len: 256,
                    f_cutoff: 0.95,
                    interpolation: SincInterpolationType::Linear,
                    oversampling_factor: 256,
                    window: WindowFunction::BlackmanHarris2,
                },
                1024,
                1
            ).expect("Failed to initialize resampler");

            let mut input_buffer = vec![Vec::with_capacity(1024)];
            let mut accumulator = Vec::new();

            loop {
                if !*is_recording_cloned.lock().unwrap() {
                    break;
                }
                
                let rx_lock = rx_cloned.lock().unwrap();
                if let Ok(data) = rx_lock.try_recv() {
                    accumulator.extend(data);
                    
                    if accumulator.len() >= 1024 {
                        let chunk = accumulator.drain(0..1024).collect::<Vec<f32>>();
                        input_buffer[0] = chunk;
                        
                        if let Ok(resampled) = resampler.process(&input_buffer, None) {
                            let mut global_buf = pcm_buffer_cloned.lock().unwrap();
                            global_buf.extend(&resampled[0]);
                            
                            // Calculate volume telemetry
                            let mut sum_sq = 0.0;
                            for &sample in &resampled[0] {
                                sum_sq += sample * sample;
                            }
                            let rms = (sum_sq / resampled[0].len() as f32).sqrt();
                            let _ = tauri::Emitter::emit(&app, "dictation_volume_level", rms);
                        }
                    }
                }
            }
        });

        Ok(())
    }

    pub fn stop_capture(&mut self) -> Vec<f32> {
        *self.is_recording.lock().unwrap() = false;
        if let Some(stream) = self.stream.take() {
            let _ = stream.pause();
        }
        
        let mut global_buf = self.pcm_buffer.lock().unwrap();
        let data = global_buf.clone();
        global_buf.clear();
        data
    }

    fn to_mono(data: &[f32], channels: u16) -> Option<Vec<f32>> {
        if channels == 1 {
            Some(data.to_vec())
        } else {
            let mut mono = Vec::with_capacity(data.len() / channels as usize);
            for chunk in data.chunks(channels as usize) {
                // Average the channels
                let sum: f32 = chunk.iter().sum();
                mono.push(sum / channels as f32);
            }
            Some(mono)
        }
    }
}
