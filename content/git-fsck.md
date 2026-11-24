# git fsck

Verify the validity and connectivity of nodes in a Git repository index. Note: Does not make any modifications.

## Usage
- `git fsck` : check the current repository
- `git fsck --tags` : list all tags found
- `git fsck --root` : list all root nodes found
- `git fsck --dangling --no-reflogs --unreachable --full` : show all unreachable and dangling objects, ignore reflogs, and perform a full integrity check
- `git fsck --connectivity-only` : check connectivity only (skip object integrity verification)

## See also
- git gc

## Categories
version-control, packages

## Links
- More information: https://git-scm.com/docs/git-fsck
- tldr page: https://tldr.inbrowser.app/pages/common/git-fsck
- Wikipedia: https://en.wikipedia.org/wiki/Git
