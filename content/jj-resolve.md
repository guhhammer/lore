# jj resolve

Resolve conflicted files with an external merge tool.

## Usage
- `jj resolve` : resolve all conflicts
- `jj resolve -l` : list all conflicts
- `jj resolve -r revset` : resolve conflicts in a given revision
- `jj resolve file1 file2 ...` : resolve conflicts in given files
- `jj resolve --tool :theirs` : resolve accepting the incoming versions
- `jj resolve --tool :ours` : resolve accepting the outgoing versions

## Categories
version-control

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-resolve
- tldr page: https://tldr.inbrowser.app/pages/common/jj-resolve
