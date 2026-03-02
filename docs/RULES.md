
---

## 5.4 docs/RULES.md
```md
# Rules

AI Dev Guardian rules implement a simple interface:

- ID
- Description
- CWE mapping
- Category
- Confidence score
- `check(file_path, content) -> Vec<Issue>`

## Current Rules

### HARD_CODED_SECRET
- Detects hardcoded secrets (API keys, tokens, passwords)
- CWE: CWE-798
- Category: Sensitive Data Exposure

### SQL_INJECTION
- Detects risky SQL string concatenation patterns
- CWE: CWE-89
- Category: Injection

## Roadmap Rule Packs (planned)
- XSS
- Command Injection
- Path Traversal
- Insecure Randomness
- Weak Crypto
- SSRF patterns
- JWT misuses
- .env leakage + credential file detection
