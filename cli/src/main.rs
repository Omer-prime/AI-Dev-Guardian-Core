mod config;

use clap::Parser;
use config::GuardianConfig;
use engine::{
    patch::apply_fixes,
    report::{generate_report, generate_sarif},
    rules::registry::RuleRegistry,
    scanner::scan_project,
};
use std::{fs, process};

#[derive(Parser, Debug)]
#[command(
    name = "ai-dev-guardian",
    version,
    about = "Local security scanner for source code"
)]
struct Args {
    /// Path to scan (project root)
    path: String,

    /// Print report as JSON to stdout
    #[arg(long)]
    json: bool,

    /// Apply autofixes where available
    #[arg(long)]
    fix: bool,

    /// Write report JSON to a file
    #[arg(long)]
    output: Option<String>,

    /// Comma-separated rule IDs to include
    #[arg(long)]
    rules: Option<String>,

    /// Comma-separated rule IDs to disable
    #[arg(long)]
    disable: Option<String>,

    /// Write SARIF report to a file
    #[arg(long)]
    sarif: Option<String>,
}

fn parse_csv_list(input: &str) -> Vec<String> {
    input
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

fn load_rule_filters_from_config() -> (Option<Vec<String>>, Option<Vec<String>>) {
    let cfg_path = ".guardian.yml";
    let Ok(config_content) = fs::read_to_string(cfg_path) else {
        return (None, None);
    };

    let Ok(config) = serde_yaml::from_str::<GuardianConfig>(&config_content) else {
        return (None, None);
    };

    let Some(rule_cfg) = config.rules else {
        return (None, None);
    };

    (rule_cfg.include, rule_cfg.exclude)
}

fn exit_code_for_report(report: &common::ScanResult) -> i32 {
    let has_critical = report
        .issues
        .iter()
        .any(|i| matches!(i.severity, common::Severity::Critical));

    let has_high = report
        .issues
        .iter()
        .any(|i| matches!(i.severity, common::Severity::High));

    if has_critical {
        1
    } else if has_high {
        2
    } else {
        0
    }
}

fn main() {
    let args = Args::parse();

    // Load rule filters from config, then override via CLI flags
    let (mut include_rules, mut exclude_rules) = load_rule_filters_from_config();

    if let Some(cli_rules) = args.rules.as_deref() {
        include_rules = Some(parse_csv_list(cli_rules));
    }

    if let Some(cli_disable) = args.disable.as_deref() {
        exclude_rules = Some(parse_csv_list(cli_disable));
    }

    let rules = RuleRegistry::filter_rules(include_rules, exclude_rules);

    // Scan + report
    let issues = scan_project(&args.path, rules);
    let report = generate_report(issues);

    // Optional exports
    if let Some(output_path) = &args.output {
        let json = serde_json::to_string_pretty(&report).expect("Failed to serialize report");
        fs::write(output_path, json).expect("Failed to write output file");
        println!("Report written to {}", output_path);
    }

    if let Some(path) = &args.sarif {
        let sarif = generate_sarif(&report);
        fs::write(path, serde_json::to_string_pretty(&sarif).unwrap())
            .expect("Failed to write SARIF file");
        println!("SARIF report written to {}", path);
    }

    if args.json {
        println!("{}", serde_json::to_string_pretty(&report).unwrap());
        process::exit(exit_code_for_report(&report)); // instead of 0
    }

    // Human-readable output
    println!("Scanning project at: {}", args.path);
    println!("Security Score: {}/100", report.score);

    for issue in &report.issues {
        println!(
            "[{:?}] {} ({}:{})",
            issue.severity, issue.description, issue.file, issue.line
        );
    }

    if args.fix {
        apply_fixes(&report.issues);
    }

    process::exit(exit_code_for_report(&report));
}
