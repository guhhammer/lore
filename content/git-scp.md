# git scp

Copy files from the current working tree to the working directory of a remote repository. Part of `git-extras`. Uses `rsync` to transfer files.

## Usage
- `git scp remote_name` : copy unstaged files to a specific remote
- `git scp remote_name HEAD` : copy staged and unstaged files to a remote
- `git scp remote_name HEAD~1` : copy files that has been changed in the last commit and any staged or unstaged files to a remote
- `git scp remote_name path/to/file1 path/to/file2 ...` : copy specific files to a remote
- `git scp remote_name path/to/directory` : copy a specific directory to a remote

## Categories
files, version-control, packages

## Links
- More information: https://github.com/tj/git-extras/blob/main/Commands.md#git-scp
- tldr page: https://tldr.inbrowser.app/pages/common/git-scp
- Wikipedia: https://en.wikipedia.org/wiki/Git
