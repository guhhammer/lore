# git guilt

Show total blame count for files with unstaged changes or calculate the change in blame between two revisions. Part of `git-extras`.

## Usage
- `git guilt` : show total blame count
- `git guilt first_revision last_revision` : calculate the change in blame between two revisions
- `git guilt -e` : show author emails instead of names
- `git guilt -w` : ignore whitespace only changes when attributing blame
- `git guilt 'git log --until "3 weeks ago" --format "%H" -n 1'` : find blame delta over the last three weeks
- `git guilt @{3.weeks.ago}` : find blame delta over the last three weeks (Git 1.8.5+)

## Categories
version-control

## Links
- More information: https://manned.org/git-guilt
- tldr page: https://tldr.inbrowser.app/pages/common/git-guilt
