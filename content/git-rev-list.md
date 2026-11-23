# git rev-list

List revisions (commits) in reverse chronological order.

## Usage
- `git rev-list HEAD` : list all commits on the current branch
- `git rev-list -n 1 HEAD -- path/to/file` : print the latest commit that changed (add/edit/remove) a specific file on the current branch
- `git rev-list --since "2019-12-01 00:00:00" branch_name` : list commits more recent than a specific date, on a specific branch
- `git rev-list --merges commit` : list all merge commits on a specific commit
- `git rev-list tag_name..HEAD --count` : print the number of commits since a specific tag

## Categories
version-control, files

## Links
- More information: https://git-scm.com/docs/git-rev-list
- tldr page: https://tldr.inbrowser.app/pages/common/git-rev-list
