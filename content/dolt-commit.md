# dolt commit

Commit staged changes to tables.

## Usage
- `dolt commit` : commit all staged changes, opening the editor specified by `$EDITOR` to enter the commit message
- `dolt commit -m "commit_message"` : commit all staged changes with the specified message
- `dolt commit -a` : stage all unstaged changes to tables before committing
- `dolt commit --date "2021-12-31T00:00:00"` : use the specified ISO 8601 commit date (defaults to current date and time)
- `dolt commit --author "author_name <author_email>"` : use the specified author for the commit
- `dolt commit --allow-empty` : allow creating an empty commit, with no changes
- `dolt commit -f` : ignore foreign key warnings

## Categories
version-control

## Links
- More information: https://docs.dolthub.com/cli-reference/cli#dolt-commit
- tldr page: https://tldr.inbrowser.app/pages/common/dolt-commit
