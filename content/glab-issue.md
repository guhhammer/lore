# glab issue

Manage GitLab issues.

## Usage
- `glab issue view issue_number` : display a specific issue
- `glab issue view issue_number -w` : display a specific issue in the default web browser
- `glab issue create --web` : create a new issue in the default web browser
- `glab issue list -P 10 -l "bug"` : list the last 10 issues with the `bug` label
- `glab issue list -c --author username` : list closed issues made by a specific user
- `glab issue reopen issue_number` : reopen a specific issue

## Links
- More information: https://gitlab.com/gitlab-org/cli/-/blob/main/docs/source/issue/_index.md
- tldr page: https://tldr.inbrowser.app/pages/common/glab-issue
