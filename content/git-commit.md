# git commit

Commit files to the repository.

## Usage
- `git commit` : open an editor to write a message and commit staged files to the repository
- `git commit -m "message"` : commit staged files to the repository with the specified message
- `git commit -F path/to/commit_message_file` : commit staged files with a message read from a file
- `git commit -a -m "message"` : auto stage all modified and deleted files and commit
- `git commit -S key_id -m "message"` : commit staged files and sign them with the specified GPG key (or the one defined in the configuration file if no `key_id` is specified)
- `git commit --amend` : update the last commit by adding the currently staged changes, changing the commit's hash, and open an editor to change the message
- `git commit path/to/file1 path/to/file2 ...` : commit only specific (already staged) files
- `git commit -m "message" -m "description"` : create a commit with the specified message and description

## Categories
version-control, files, packages

## Links
- More information: https://git-scm.com/docs/git-commit
- tldr page: https://tldr.inbrowser.app/pages/common/git-commit
