# gh project

Work with GitHub Projects.

## Usage
- `gh project ls` : list projects owned by the currently authenticated user
- `gh project ls --owner owner` : list projects owned by a specific user or organization
- `gh project view number --owner owner` : view a project by number
- `gh project create --owner owner --title project_title` : create a new project
- `gh project item-add number --owner owner --url issue_or_pr_url` : add an item (issue or pull request) to a project
- `gh project item-list number --owner owner` : list items in a project
- `gh project close number --owner owner` : close a project

## Categories
development, network

## Links
- More information: https://cli.github.com/manual/gh_project
- tldr page: https://tldr.inbrowser.app/pages/common/gh-project
