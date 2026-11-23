# gita

Manage multiple Git repositories side by side.

## Usage
- `gita ll` : display a summary of a[ll] registered repositories
- `gita st` : show the [st]atus of all registered repositories
- `gita add path/to/repo1 path/to/repo2 ...` : register one or more repositories with gita
- `gita pull` : pull updates for all registered repositories
- `gita super repo_name git_command` : run a Git command on a specific repository
- `gita group add -n group_name repo1 repo2 ... && gita context group_name` : create a group of repositories and set it as the active context
- `gita fetch repo1 repo2 ...|group1 group2 ...` : fetch updates for one or more repositories or groups
- `gita freeze` : display information of all repositories

## See also
- git

## Categories
version-control, users

## Links
- More information: https://github.com/nosarthur/gita
- tldr page: https://tldr.inbrowser.app/pages/common/gita
