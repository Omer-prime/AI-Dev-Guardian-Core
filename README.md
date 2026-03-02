# AI Dev Guardian (Core)

AI Dev Guardian is an open-source **local security scanner + auto-fix assistant** for source code.
It scans your project for high-impact security risks, outputs **SARIF for GitHub Code Scanning**, and can apply **safe autofixes** where available.

> Goal: not just “detect”. AI Dev Guardian is built to **detect → explain → patch → verify**.

## Features (Core)
- ✅ Fast local scanning (parallel file scan)
- ✅ Rule engine (extensible rules)
- ✅ `.guardian.yml` config (include/exclude rules)
- ✅ Human output + JSON report
- ✅ SARIF export (GitHub Code Scanning)
- ✅ Exit codes for CI gating (fail builds on high/critical)
- ✅ Safe auto-fixes (with `.bak` backups)
- ✅ Optional HTTP API (`/scan`) for local integration

## Quick Start

### 1) Build from source
```bash
git clone https://github.com/Omer-prime/AI-Dev-Guardian-Core.git
cd AI-Dev-Guardian-Core

# Run the CLI against a project folder
cargo run -p cli -- . 
