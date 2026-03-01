use common::Issue;
use std::collections::HashMap;
use std::fs;
use std::io::Write;

pub fn apply_fixes(issues: &Vec<Issue>) {
    let mut file_map: HashMap<String, Vec<&Issue>> = HashMap::new();

    // Group issues by file
    for issue in issues {
        file_map
            .entry(issue.file.clone())
            .or_insert_with(Vec::new)
            .push(issue);
    }

    for (file_path, file_issues) in file_map {
        if let Ok(content) = fs::read_to_string(&file_path) {

            let backup_path = format!("{}.bak", file_path);
            fs::copy(&file_path, &backup_path).ok();

            let mut lines: Vec<String> =
                content.lines().map(|s| s.to_string()).collect();

            for issue in file_issues {
                if let Some(fix) = &issue.fix {
                    if let Some(example) = &fix.replacement_example {
                        if issue.line - 1 < lines.len() {
                            lines[issue.line - 1] = example.clone();
                        }
                    }
                }
            }

            let new_content = lines.join("\n");

            let mut file =
                fs::File::create(&file_path).unwrap();
            file.write_all(new_content.as_bytes()).unwrap();

            println!("Applied all fixes to {}", file_path);
        }
    }
}