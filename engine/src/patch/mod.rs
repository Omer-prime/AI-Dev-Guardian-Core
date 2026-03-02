use common::Issue;
use std::collections::HashMap;
use std::fs;
use std::io::Write;

pub fn apply_fixes(issues: &[Issue]) {
    let mut file_map: HashMap<String, Vec<&Issue>> = HashMap::new();

    // Group issues by file
    for issue in issues {
        file_map.entry(issue.file.clone()).or_default().push(issue);
    }

    for (file_path, file_issues) in file_map {
        let Ok(content) = fs::read_to_string(&file_path) else {
            continue;
        };

        let backup_path = format!("{}.bak", file_path);
        let _ = fs::copy(&file_path, &backup_path);

        let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

        for issue in file_issues {
            let example = issue
                .fix
                .as_ref()
                .and_then(|fix| fix.replacement_example.as_ref());

            if let Some(example) = example {
                let idx = issue.line.saturating_sub(1);
                if idx < lines.len() {
                    lines[idx] = example.clone();
                }
            }
        }

        let new_content = lines.join("\n");

        let mut file = fs::File::create(&file_path).unwrap();
        file.write_all(new_content.as_bytes()).unwrap();

        println!("Applied all fixes to {}", file_path);
    }
}
