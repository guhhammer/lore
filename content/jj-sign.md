# jj sign

Cryptographically sign revisions. Requires configuring a commit signing backend.

## Usage
- `jj sign` : sign revisions configured by the `revsets.sign` setting
- `jj sign -r revset` : sign a specific revision
- `jj sign -r revset1 -r revset2` : sign multiple revisions
- `jj sign --key key_id -r revset` : sign revisions using a specific signing key

## See also
- jj unsign

## Categories
version-control

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-sign
- tldr page: https://tldr.inbrowser.app/pages/common/jj-sign
