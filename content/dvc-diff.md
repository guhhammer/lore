# dvc diff

Show changes in DVC tracked file and directories.

## Usage
- `dvc diff commit_hash/tag/branch` : compare DVC tracked files from different Git commits, tags, and branches w.r.t the current workspace
- `dvc diff revision1 revision2` : compare the changes in DVC tracked files from 1 Git commit to another
- `dvc diff --show-hash commit` : compare DVC tracked files, along with their latest hash
- `dvc diff --show-json --show-hash commit` : compare DVC tracked files, displaying the output as JSON
- `dvc diff --show-md --show-hash commit` : compare DVC tracked files, displaying the output as Markdown

## Categories
version-control, files, security

## Links
- More information: https://doc.dvc.org/command-reference/diff
- tldr page: https://tldr.inbrowser.app/pages/common/dvc-diff
