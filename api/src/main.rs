use axum::{
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

use engine::scanner::scan_project;
use engine::report::generate_report;
use engine::rules::registry::RuleRegistry;

#[derive(Deserialize)]
struct ScanRequest {
    path: String,
}

#[derive(Serialize)]
struct ScanResponse {
    score: u8,
    issues: Vec<common::Issue>,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/scan", post(scan_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("API running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}

async fn scan_handler(
    Json(payload): Json<ScanRequest>,
) -> Json<ScanResponse> {

    let rules = RuleRegistry::all_rules();
    let issues = scan_project(&payload.path, rules);
    let report = generate_report(issues);

    Json(ScanResponse {
        score: report.score,
        issues: report.issues,
    })
}