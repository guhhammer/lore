# hg status

Show files that have changed in the working directory.

## Usage
- `hg status` : display the status of changed files
- `hg status -m` : display only modified files
- `hg status -a` : display only added files
- `hg status -r` : display only removed files
- `hg status -d` : display only deleted (but tracked) files
- `hg status --rev revision` : display changes in the working directory compared to a specified changeset
- `hg status -I pattern` : display only files matching a specified `glob` pattern
- `hg status -X pattern` : display files, excluding those that match a specified `glob` pattern

## Categories
desktop, files, text

## Links
- More information: https://www.mercurial-scm.org/help/commands/status
- tldr page: https://tldr.inbrowser.app/pages/common/hg-status
