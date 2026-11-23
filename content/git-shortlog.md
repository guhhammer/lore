# git shortlog

Summarize the `git log` output.

## Usage
- `git shortlog` : view a summary of all the commits made, grouped alphabetically by author name
- `git shortlog -n` : view a summary of all the commits made, sorted by the number of commits made
- `git shortlog -c` : view a summary of all the commits made, grouped by the committer identities (name and email)
- `git shortlog HEAD~5..HEAD` : view a summary of the last 5 commits (i.e. specify a revision range)
- `git shortlog -sne` : view all users, emails, and the number of commits in the current branch
- `git shortlog -sne --all` : view all users, emails, and the number of commits in all branches

## Categories
version-control, monitoring

## Links
- More information: https://git-scm.com/docs/git-shortlog
- tldr page: https://tldr.inbrowser.app/pages/common/git-shortlog
