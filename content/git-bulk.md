# git bulk

Execute operations on multiple Git repositories. Part of `git-extras`.

## Usage
- `git bulk --addcurrent workspace_name` : register the current directory as a workspace
- `git bulk --addworkspace workspace_name /path/to/repository` : register a workspace for bulk operations
- `git bulk --addworkspace workspace_name /path/to/parent_directory --from remote_repository_location` : clone a repository inside a specific directory, then register the repository as a workspace
- `git bulk --addworkspace workspace_name /path/to/root_directory --from /path/to/file` : clone repositories from a newline-separated list of remote locations, then register them as workspaces
- `git bulk --listall` : list all registered workspaces
- `git bulk command command_arguments` : run a Git command on the repositories of the current workspace
- `git bulk --removeworkspace workspace_name` : remove a specific workspace
- `git bulk --purge` : remove all workspaces

## Categories
version-control, files, packages

## Links
- More information: https://github.com/tj/git-extras/blob/main/Commands.md#git-bulk
- tldr page: https://tldr.inbrowser.app/pages/common/git-bulk
- Wikipedia: https://en.wikipedia.org/wiki/Git
