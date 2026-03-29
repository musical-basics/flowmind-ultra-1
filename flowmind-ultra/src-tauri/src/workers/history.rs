use similar::{ChangeTag, TextDiff};
use std::io::Read;

pub fn generate_unified_diff(original: &str, patched: &str) -> Vec<u8> {
    let diff = TextDiff::from_lines(original, patched);
    let mut diff_str = String::new();
    
    for change in diff.iter_all_changes() {
        let sign = match change.tag() {
            ChangeTag::Delete => "-",
            ChangeTag::Insert => "+",
            ChangeTag::Equal => " ",
        };
        diff_str.push_str(&format!("{}{}", sign, change.value()));
    }
    
    // Compress with zstd
    zstd::encode_all(diff_str.as_bytes(), 3).unwrap_or_default()
}

pub fn apply_patch(original: &str, compressed_diff: &[u8]) -> String {
    let decompressed = zstd::decode_all(compressed_diff).unwrap_or_default();
    let diff_str = String::from_utf8_lossy(&decompressed);
    
    let mut result = String::new();
    for line in diff_str.lines() {
        if line.starts_with('+') || line.starts_with(' ') {
            result.push_str(&line[1..]);
            result.push('\n');
        }
    }
    result
}
