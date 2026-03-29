use ignore::WalkBuilder;
use std::fs;
use std::path::Path;

pub fn flatten_workspace<P: AsRef<Path>>(dir: P, ignored_dirs: Option<Vec<String>>) -> Result<String, String> {
    let mut output = String::new();
    let root = dir.as_ref();
    
    let mut builder = WalkBuilder::new(root);
    builder.hidden(false);
    
    if let Some(ignored) = ignored_dirs {
        builder.filter_entry(move |entry| {
            if let Some(name) = entry.file_name().to_str() {
                if ignored.contains(&name.to_string()) {
                    return false;
                }
            }
            true
        });
    }

    for result in builder.build() {
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
