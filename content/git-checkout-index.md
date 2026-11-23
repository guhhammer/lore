# git checkout-index

Copy files from the index to the working tree.

## Usage
- `git checkout-index -a` : restore any files deleted since the last commit
- `git checkout-index -a -f` : restore any files deleted or changed since the last commit
- `git checkout-index -a -f -n` : restore any files changed since the last commit, ignoring any files that were deleted
- `git checkout-index -a -f --prefix path/to/export_directory/` : export a copy of the entire tree at the last commit to the specified directory (the trailing slash is important)

## Categories
version-control, files

## Links
- More information: https://git-scm.com/docs/git-checkout-index
- tldr page: https://tldr.inbrowser.app/pages/common/git-checkout-index
