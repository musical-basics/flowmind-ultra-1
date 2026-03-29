use regex::Regex;

pub fn strip_ansi(raw: &[u8]) -> String {
    let clean = strip_ansi_escapes::strip(raw);
    String::from_utf8_lossy(&clean).into_owned()
}

pub fn is_prompt_waiting(clean_text: &str) -> bool {
    if let Ok(re) = Regex::new(r"[\$>\❯]\s*$") {
        re.is_match(clean_text)
    } else {
        false
    }
}
