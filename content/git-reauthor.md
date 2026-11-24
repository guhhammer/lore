# git reauthor

Change details about an author identity. Since this command rewrites the Git history, `--force` will be needed when pushing next time. Part of `git-extras`.

## Usage
- `git reauthor -o old@example.com -e new@example.com -n "name"` : change an author's email and name across the whole Git repository
- `git reauthor -o old@example.com -c` : change the email and name to the ones defined in the Git config
- `git reauthor -a -e name@example.com -n name` : change the email and name of all commits, regardless of their original author

## Categories
version-control, shell

## Links
- More information: https://github.com/tj/git-extras/blob/main/Commands.md#git-reauthor
- tldr page: https://tldr.inbrowser.app/pages/common/git-reauthor
- Wikipedia: https://en.wikipedia.org/wiki/Git
