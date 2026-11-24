# git filter-branch

Change branch history, like removing files.

## Usage
- `git filter-branch --tree-filter 'rm -f file' HEAD` : remove a file from all commits
- `git filter-branch --env-filter 'GIT_AUTHOR_EMAIL=new_email' HEAD` : update author email
- `git filter-branch --tree-filter 'rm -rf path/to/folder' HEAD` : delete a folder from history

## Categories
version-control, files, shell

## Links
- More information: https://git-scm.com/docs/git-filter-branch
- tldr page: https://tldr.inbrowser.app/pages/common/git-filter-branch
- Wikipedia: https://en.wikipedia.org/wiki/Git
