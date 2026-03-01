use rayon::prelude::*;
use walkdir::WalkDir;
use std::fs;
use std::collections::HashSet;
use common::Issue;
use crate::rules::SecurityRule;

pub fn scan_project(
    path: &str,
    rules: Vec<Box<dyn SecurityRule>>,
) -> Vec<Issue> {

    // 1️⃣ Collect all valid source files first
    let files: Vec<_> = WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            let path = e.path().to_string_lossy();

            e.file_type().is_file()
                && !path.contains("node_modules")
                && !path.contains("venv")
                && !path.contains(".git")
                && !path.ends_with(".bak")
                && (
                    path.ends_with(".js")
                    || path.ends_with(".ts")
                    || path.ends_with(".jsx")
                    || path.ends_with(".tsx")
                    || path.ends_with(".py")
                )
        })
        .map(|e| e.into_path())
        .collect();

    // 2️⃣ Parallel scan using Rayon
    let issues: Vec<Issue> = files
        .par_iter()
        .flat_map(|file_path| {
            if let Ok(content) = fs::read_to_string(file_path) {

                rules
                    .iter()
                    .flat_map(|rule| {
                        rule.check(
                            file_path.to_string_lossy().as_ref(),
                            &content,
                        )
                    })
                    .collect::<Vec<Issue>>()

            } else {
                Vec::new()
            }
        })
        .collect();

    // 3️⃣ Deduplicate issues (enterprise-grade behavior)
    let mut seen = HashSet::new();

    issues
        .into_iter()
        .filter(|issue| {
            let key = format!(
                "{}:{}:{}",
                issue.file, issue.line, issue.id
            );
            seen.insert(key)
        })
        .collect()
}