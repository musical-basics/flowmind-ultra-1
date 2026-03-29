use ignore::WalkBuilder;
use std::fs;
use std::path::Path;

pub fn flatten_workspace<P: AsRef<Path>>(dir: P) -> Result<String, String> {
    let mut output = String::new();
    let root = dir.as_ref();
    for result in WalkBuilder::new(root).hidden(false).build() {
        match result {
            Ok(entry) => {
                if !entry.file_type().map_or(false, |ft| ft.is_dir()) {
                    let path = entry.path();
                    let relative = path.strip_prefix(root).unwrap_or(path);
                    output.push_str(&format!("\n--- File: {} ---\n```\n", relative.display()));
                    if let Ok(content) = fs::read_to_string(path) {
                        output.push_str(&content);
                    } else {
                        output.push_str("<binary or unreadable>");
                    }
                    output.push_str("\n```\n");
                }
            }
            Err(_) => continue,
        }
    }
    Ok(output)
}
