# jj bookmark

Manage bookmarks in a `jj` repository. When using a Git backend, bookmarks correspond to Git branches.

## Usage
- `jj b c -r revision name` : create a new bookmark at the given revision
- `jj b l` : list all existing bookmarks and their targets
- `jj b m -t revision name` : move an existing bookmark to another revision
- `jj b t name@remote` : track given remote bookmarks
- `jj b d name` : delete a bookmark, and propagate the deletion to remotes on the next push
- `jj b f name` : forget a bookmark locally, without marking its deletion to be pushed

## Categories
version-control, packages

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-bookmark
- tldr page: https://tldr.inbrowser.app/pages/common/jj-bookmark
