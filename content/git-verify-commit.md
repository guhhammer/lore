# git verify-commit

Check for GPG verification of commits. If no commits are verified, nothing will be printed, regardless of options specified.

## Usage
- `git verify-commit commit_hash1 optional_commit_hash2 ...` : check commits for a GPG signature
- `git verify-commit commit_hash1 optional_commit_hash2 ... -v` : check commits for a GPG signature and show details of each commit
- `git verify-commit commit_hash1 optional_commit_hash2 ... --raw` : check commits for a GPG signature and print the raw details

## Categories
version-control, security

## Links
- More information: https://git-scm.com/docs/git-verify-commit
- tldr page: https://tldr.inbrowser.app/pages/common/git-verify-commit
