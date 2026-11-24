# git log

Show a history of commits.

## Usage
- `git log` : show the sequence of commits starting from the current one, in reverse chronological order of the Git repository in the current working directory
- `git log -p path/to/file_or_directory` : show the history of a particular file or directory, including differences
- `git log --stat` : show an overview of which file(s) changed in each commit
- `git log --oneline --graph` : show a graph of commits in the current branch using only the first line of each commit message
- `git log --oneline --decorate --all --graph` : show a graph of all commits, tags, and branches in the entire repo
- `git log -i --grep search_string` : show only commits with messages that include a specific string, ignoring case
- `git log -n n --author "author"` : show the last `n` number of commits from a certain author
- `git log --before "2017-01-29" --after "2017-01-17"` : show commits between two dates (yyyy-mm-dd)

## Categories
version-control, monitoring, files

## Links
- More information: https://git-scm.com/docs/git-log
- tldr page: https://tldr.inbrowser.app/pages/common/git-log
- Wikipedia: https://en.wikipedia.org/wiki/Git
