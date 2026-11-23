# dvc fetch

Download DVC tracked files and directories from a remote repository.

## Usage
- `dvc fetch` : fetch the latest changes from the default remote upstream repository (if set)
- `dvc fetch -r remote_name` : fetch changes from a specific remote upstream repository
- `dvc fetch target1 target2 ...` : fetch the latest changes for a specific targets
- `dvc fetch -a -T` : fetch changes for all branch and tags
- `dvc fetch -A` : fetch changes for all commits

## Categories
network, packages, files

## Links
- More information: https://doc.dvc.org/command-reference/fetch
- tldr page: https://tldr.inbrowser.app/pages/common/dvc-fetch
