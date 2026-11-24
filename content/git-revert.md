# git revert

Create new commits which reverse the effect of earlier ones.

## Usage
- `git revert HEAD` : revert the most recent commit
- `git revert HEAD~4` : revert the 5th last commit
- `git revert 0c01a9` : revert a specific commit
- `git revert branch_name~5..branch_name~2` : revert multiple commits
- `git revert -n 0c01a9..9a1743` : don't create new commits, just change the working tree
- `git revert --abort` : cancel a Git revert after a merge conflict

## Categories
version-control

## Links
- More information: https://git-scm.com/docs/git-revert
- tldr page: https://tldr.inbrowser.app/pages/common/git-revert
- Wikipedia: https://en.wikipedia.org/wiki/Git
