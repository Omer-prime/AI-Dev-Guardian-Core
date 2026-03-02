# GitHub Actions (Free Bot)

This runs AI Dev Guardian on PRs and uploads SARIF to GitHub Code Scanning.

## Step 1 — Add workflow file
In your target repo, create:

`.github/workflows/ai-dev-guardian.yml`

```yaml
name: AI Dev Guardian

on:
  pull_request:
  push:
    branches: [ main ]

permissions:
  contents: read
  security-events: write

jobs:
  scan:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - uses: dtolnay/rust-toolchain@stable

      - name: Build AI Dev Guardian
        run: cargo build --release

      - name: Run scan (SARIF)
        run: cargo run -p cli -- . --sarif report.sarif

      - name: Upload SARIF
        uses: github/codeql-action/upload-sarif@v3
        with:
          sarif_file: report.sarif
