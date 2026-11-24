# git apply

Apply a patch to files and/or to the index without creating a commit.

## Usage
- `git apply -v path/to/file` : print messages about the patched files
- `git apply --index path/to/file` : apply and add the patched files to the index
- `curl -L https://example.com/file.patch | git apply` : apply a remote patch file
- `git apply --stat --apply path/to/file` : output diffstat for the input and apply the patch
- `git apply -R path/to/file` : apply the patch in reverse
- `git apply --cached path/to/file` : store the patch result in the index without modifying the working tree

## See also
- git am

## Categories
version-control, files, network

## Links
- More information: https://git-scm.com/docs/git-apply
- tldr page: https://tldr.inbrowser.app/pages/common/git-apply
- Wikipedia: https://en.wikipedia.org/wiki/Git
