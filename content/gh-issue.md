# gh issue

Manage GitHub issues.

## Usage
- `gh issue view issue_number` : display a specific issue
- `gh issue view issue_number -w` : display a specific issue in the default web browser
- `gh issue new -w` : create a new issue in the default web browser
- `gh issue ls -L 10 -l "bug"` : list the last 10 issues with the `bug` label
- `gh issue ls -s closed -A username` : list closed issues made by a specific user
- `gh issue status -R owner/repository` : display the status of issues relevant to the user, in a specific repository
- `gh issue reopen issue_number` : reopen a specific issue

## Categories
desktop

## Links
- More information: https://cli.github.com/manual/gh_issue
- tldr page: https://tldr.inbrowser.app/pages/common/gh-issue
