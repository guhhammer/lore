# jj gerrit

Interact with Gerrit to upload changes for code review, or update existing changes.

## Usage
- `jj gerrit upload` : upload the current change (or its parent if the working copy has no description)
- `jj gerrit upload -r revset` : upload a specific revision
- `jj gerrit upload -r revset1 -r revset2` : upload multiple specific revsets
- `jj gerrit upload -r revset -b branch` : upload a revision targeting a specific remote branch for merging
- `jj gerrit upload --remote remote` : upload to a specific Gerrit remote
- `jj gerrit upload --reviewer reviewer@example.com --cc cc@example.com` : upload and add reviewers and CC recipients by email (flags can be repeated)
- `jj gerrit upload --quiet` : upload while suppressing non-essential output
- `jj gerrit upload -n` : simulate an upload, showing what would be uploaded without pushing to Gerrit

## See also
- jj git push

## Categories
network

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-gerrit-upload
- tldr page: https://tldr.inbrowser.app/pages/common/jj-gerrit
