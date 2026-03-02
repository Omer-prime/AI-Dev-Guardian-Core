mod config; // Add this line to declare the config module

use clap::Parser;
use common::ScanResult;
use config::GuardianConfig;
use dotenv::dotenv;
use engine::patch::apply_fixes;
use engine::report::generate_report;
use engine::rules::registry::RuleRegistry;
use engine::scanner::scan_project;
use serde_json;
use sqlx::PgPool;
use std::env;
use std::fs; // Add this to import std::fs
use std::process;
use tokio;

#[derive(Parser)]
#[command(name = "ai-dev-guardian")]
struct Args {
    path: String,

    #[arg(long)]
    json: bool,

    #[arg(long)]
    fix: bool,

    #[arg(long)]
    output: Option<String>,

    #[arg(long)]
    rules: Option<String>,

    #[arg(long)]
    disable: Option<String>,

    #[arg(long)]
    sarif: Option<String>,
}

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load environment variables from .env file

    // Fetch DATABASE_URL from the environment
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Print the DATABASE_URL to check if it was loaded correctly
    println!("Using DATABASE_URL: {}", database_url);

    let args = Args::parse();

    // Load config file if exists
    let mut include_rules = None;
    let mut exclude_rules = None;

    if let Ok(config_content) = fs::read_to_string(".guardian.yml") {
        if let Ok(config) = serde_yaml::from_str::<GuardianConfig>(&config_content) {
            if let Some(rule_cfg) = config.rules {
                include_rules = rule_cfg.include;
                exclude_rules = rule_cfg.exclude;
            }
        }
    }

    // CLI overrides config
    if let Some(cli_rules) = args.rules {
        include_rules = Some(cli_rules.split(',').map(|s| s.trim().to_string()).collect());
    }

    if let Some(cli_disable) = args.disable {
        exclude_rules = Some(
            cli_disable
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
        );
    }

    // Get filtered rules
    let rules = RuleRegistry::filter_rules(include_rules, exclude_rules);

    // Set up PostgreSQL connection
    let _pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    // Run scan
    let issues = scan_project(&args.path, rules);
    let report = generate_report(issues);

    // Save report to DB
    if let Err(e) = save_report_to_db(&report).await {
        eprintln!("Failed to save report to DB: {}", e);
    }

    // Output file
    if let Some(output_path) = &args.output {
        let json = serde_json::to_string_pretty(&report).unwrap();
        fs::write(output_path, json).unwrap();
        println!("Report written to {}", output_path);
    }

    // SARIF Export
    if let Some(path) = &args.sarif {
        let sarif = engine::report::generate_sarif(&report);
        std::fs::write(path, serde_json::to_string_pretty(&sarif).unwrap())
            .expect("Failed to write SARIF file");

        println!("SARIF report written to {}", path);
    }

    // JSON-only mode
    if args.json {
        println!("{}", serde_json::to_string_pretty(&report).unwrap());
        process::exit(0);
    }

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

    let has_critical = report
        .issues
        .iter()
        .any(|i| matches!(i.severity, common::Severity::Critical));

    let has_high = report
        .issues
        .iter()
        .any(|i| matches!(i.severity, common::Severity::High));

    if has_critical {
        process::exit(1);
    } else if has_high {
        process::exit(2);
    } else {
        process::exit(0);
    }
}

async fn save_report_to_db(report: &ScanResult) -> Result<(), sqlx::Error> {
    let pool = PgPool::connect("postgres://postgres:yourpassword@localhost/ai_dev_guardian")
        .await
        .expect("Failed to connect to the database");

    // Insert the report into the database
    for issue in &report.issues {
        sqlx::query!(
            "INSERT INTO scans (file_path, issue_id, severity, description, line) VALUES ($1, $2, $3, $4, $5)",
            issue.file,
            issue.id,
            format!("{:?}", issue.severity),
            issue.description,
            issue.line as i32  // Cast `usize` to `i32`
        )
        .execute(&pool)
        .await?;
    }

    Ok(())
}
