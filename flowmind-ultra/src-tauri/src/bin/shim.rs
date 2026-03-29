use std::env;
use std::io::{self, Write};
use std::process::{Command, Stdio};

const OSC_READY: &str = "\x1b]633;A\x07";
const OSC_EXIT_PREFIX: &str = "\x1b]633;D;";
const SHIM_LAUNCH_ERROR_MARKER: &str = "SHIM_LAUNCH_ERROR";

fn force_utf8_console() {
    #[cfg(windows)]
    {
        // TODO: Ensure Windows uses UTF-8 CodePage 65001.
    }
}

fn main() {
    force_utf8_console();
    
    let mut args = env::args();
    let _shim = args.next();
    
    let target = match args.next() {
        Some(value) => value,
        None => {
            eprintln!("{}: no target command", SHIM_LAUNCH_ERROR_MARKER);
            std::process::exit(101);
        }
    };
    
    let target_args: Vec<String> = args.collect();

    // Broadcast READY to Flowmind parent.
    print!("{}", OSC_READY);
    let _ = io::stdout().flush();

    let child = Command::new(&target)
        .args(&target_args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn();

    match child {
        Ok(mut child) => {
            let status = match child.wait() {
                Ok(status) => status,
                Err(err) => {
                    eprintln!("{}: wait error='{}'", SHIM_LAUNCH_ERROR_MARKER, err);
                    std::process::exit(103);
                }
            };
            let code = status.code().unwrap_or(0);
            
            // Broadcast EXIT CODE to Flowmind parent.
            print!("{}{}\x07", OSC_EXIT_PREFIX, code);
            let _ = io::stdout().flush();
            std::process::exit(code);
        }
        Err(err) => {
            eprintln!("{}: command='{}' error='{}'", SHIM_LAUNCH_ERROR_MARKER, target, err);
            std::process::exit(102);
        }
    }
}
