# gitleaks

Detect secrets and API keys leaked in Git repositories.

## Usage
- `gitleaks detect --repo-url https://github.com/username/repository.git` : scan a remote repository
- `gitleaks detect -s path/to/repository` : scan a local directory
- `gitleaks detect -s path/to/repository --report path/to/report.json` : output scan results to a JSON file
- `gitleaks detect -s path/to/repository --config-path path/to/config.toml` : use a custom rules file
- `gitleaks detect -s path/to/repository --log-opts --since=commit_id` : start scanning from a specific commit
- `gitleaks protect --staged` : scan uncommitted changes before a commit
- `gitleaks protect --staged --verbose` : display verbose output indicating which parts were identified as leaks during the scan

## Categories
files, packages, version-control

## Links
- More information: https://github.com/gitleaks/gitleaks#usage
- tldr page: https://tldr.inbrowser.app/pages/common/gitleaks
