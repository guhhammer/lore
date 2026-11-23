# protector

Protect or unprotect branches on GitHub repositories.

## Usage
- `protector branches_regex -repos organization/repository` : protect branches of a GitHub repository (create branch protection rules)
- `protector -dry-run branches_regex -repos organization/repository` : simulate what would be protected without actually doing it
- `protector -free branches_regex -repos organization/repository` : free branches of a GitHub repository (delete branch protection rules)

## Categories
packages

## Links
- More information: https://github.com/jcgay/protector#usage
- tldr page: https://tldr.inbrowser.app/pages/common/protector
