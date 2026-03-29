pub fn slice_spec(markdown: &str, header: &str) -> Option<String> {
    let header_prefix = format!("# {}", header);
    let h2_prefix = format!("## {}", header);
    let h3_prefix = format!("### {}", header);
    let mut capturing = false;
    let mut result = Vec::new();

    for line in markdown.lines() {
        if line.starts_with(&header_prefix) || line.starts_with(&h2_prefix) || line.starts_with(&h3_prefix) {
            capturing = true;
            result.push(line);
            continue;
        }
        if capturing && line.starts_with('#') {
            break;
        }
        if capturing {
            result.push(line);
        }
    }

    if result.is_empty() {
        None
    } else {
        Some(result.join("\n").trim().to_string())
    }
}
