# jj sparse

Manage which paths from the working-copy commit are present in the working copy.

## Usage
- `jj sparse list` : list the patterns currently present in the working copy
- `jj sparse edit` : start an editor to update the sparse patterns
- `jj sparse reset` : reset the patterns to include all files in the working copy
- `jj sparse set --clear` : exclude all files from the working copy
- `jj sparse set --clear --add path/to/directory` : update patterns to only include specified paths
- `jj sparse set --add path/to/directory` : add a path to the working copy
- `jj sparse set --add path/to/directory1 --add path/to/directory2` : add multiple paths to the working copy
- `jj sparse set --remove path/to/directory` : remove a path from the working copy

## Categories
files, version-control

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-sparse
- tldr page: https://tldr.inbrowser.app/pages/common/jj-sparse
