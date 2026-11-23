# jj workspace

Manage Jujutsu workspaces.

## Usage
- `jj workspace list` : list all workspaces attached to the repository
- `jj workspace add path/to/directory` : create a new workspace at the specified path
- `jj workspace add --name workspace_name -r revision path/to/directory` : create a new workspace with a specific name and parent revision
- `jj workspace root` : print the root directory of the current workspace
- `jj workspace root --name workspace_name` : print the root directory of a specific workspace
- `jj workspace rename new_name` : rename the current workspace
- `jj workspace forget workspace_name` : stop tracking a workspace without deleting its files
- `jj workspace update-stale` : update a workspace whose working copy has become stale

## Categories
files

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-workspace
- tldr page: https://tldr.inbrowser.app/pages/common/jj-workspace
