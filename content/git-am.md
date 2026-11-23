# git am

Apply patch files and create a commit. Useful when receiving commits via email.

## Usage
- `git am path/to/file.patch` : apply and commit changes following a local patch file
- `curl -L https://example.com/file.patch | git am` : apply and commit changes following a remote patch file
- `git am --abort` : abort the process of applying a patch file
- `git am --reject path/to/file.patch` : apply as much of a patch file as possible, saving failed hunks to reject files

## See also
- git format-patch

## Categories
version-control, files, network

## Links
- More information: https://git-scm.com/docs/git-am
- tldr page: https://tldr.inbrowser.app/pages/common/git-am
