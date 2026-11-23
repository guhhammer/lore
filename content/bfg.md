# bfg

Remove large files or passwords from Git history like `git filter-branch`. Note: If your repository is connected to a remote, you will need to force push to it.

## Usage
- `bfg --delete-files file_with_sensitive_data` : remove a file with sensitive data but leave the latest commit untouched
- `bfg --replace-text path/to/file.txt` : remove all text mentioned in the specified file wherever it can be found in the repository's history

## Categories
version-control, files, packages

## Links
- More information: https://rtyley.github.io/bfg-repo-cleaner/
- tldr page: https://tldr.inbrowser.app/pages/common/bfg
