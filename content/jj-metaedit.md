# jj metaedit

Modify the metadata of a revision without changing its content.

## Usage
- `jj metaedit --update-author` : update the author of the working-copy commit to the configured user
- `jj metaedit --author "name <email>"` : set a specific author on the working-copy commit
- `jj metaedit --update-author-timestamp -r revset` : update the author date of a revision to the current time
- `jj metaedit --author-timestamp timestamp -r revset` : set the author date of a revision to a specific timestamp
- `jj metaedit -m message -r revset` : update the change description of a revision without opening an editor
- `jj metaedit --update-change-id -r revset` : generate a new change ID for a revision
- `jj metaedit --force-rewrite -r revset` : force a commit to be rewritten, updating the committer details and timestamp
- `jj metaedit --update-author revset1 revset2` : update the author of multiple revisions to the configured user

## See also
- jj describe

## Categories
version-control

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-metaedit
- tldr page: https://tldr.inbrowser.app/pages/common/jj-metaedit
