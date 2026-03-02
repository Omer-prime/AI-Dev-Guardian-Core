
---

## 5.6 CONTRIBUTING.md
```md
# Contributing

Thanks for your interest in contributing to AI Dev Guardian!

## Development setup
```bash
git clone https://github.com/Omer-prime/AI-Dev-Guardian-Core.git
cd AI-Dev-Guardian-Core
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test
