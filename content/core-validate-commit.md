# core-validate-commit

Validate commit messages for Node.js core.

## Usage
- `core-validate-commit` : validate the current commit
- `core-validate-commit commit_hash` : validate a specific commit
- `git rev-list commit_hash..HEAD | xargs core-validate-commit` : validate a range of commits
- `core-validate-commit -l` : list all validation rules
- `core-validate-commit -ls` : list all valid Node.js subsystems
- `core-validate-commit -t` : validate the current commit formatting the output in tap format
- `core-validate-commit -h` : display help

## Categories
version-control

## Links
- More information: https://github.com/nodejs/core-validate-commit
- tldr page: https://tldr.inbrowser.app/pages/common/core-validate-commit
