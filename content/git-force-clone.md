# git force-clone

Get the basic functionality of `git clone`, but if the destination Git repository already exists it will force-reset it to resemble a clone of the remote. Part of `git-extras`.

## Usage
- `git force-clone remote_repository_location path/to/directory` : clone a Git repository into a new directory
- `git force-clone -b branch_name remote_repository_location path/to/directory` : clone a Git repository into a new directory, checking out a specific branch

## Categories
version-control, files, packages

## Links
- More information: https://github.com/tj/git-extras/blob/main/Commands.md#git-force-clone
- tldr page: https://tldr.inbrowser.app/pages/common/git-force-clone
