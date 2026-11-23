# gh issue create

Create GitHub issues on a repository.

## Usage
- `gh issue new` : create a new issue against the current repository interactively
- `gh issue new -l "bug"` : create a new issue with the `bug` label interactively
- `gh issue new -a user1,user2,...` : create a new issue interactively and assign it to the specified users
- `gh issue new -t "title" -b "body" -a "@me"` : create a new issue with a title and body, and assign it to the current user
- `gh issue new -F path/to/file` : create a new issue interactively, reading the body text from a file
- `gh issue new -w` : create a new issue in the default web browser
- `gh issue new -h` : display help

## Categories
packages, files

## Links
- More information: https://cli.github.com/manual/gh_issue_create
- tldr page: https://tldr.inbrowser.app/pages/common/gh-issue-create
