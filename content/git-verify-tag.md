# git verify-tag

Check for GPG verification of tags. If a tag wasn't signed, an error will occur.

## Usage
- `git verify-tag tag1 optional_tag2 ...` : check tags for a GPG signature
- `git verify-tag tag1 optional_tag2 ... -v` : check tags for a GPG signature and show details for each tag
- `git verify-tag tag1 optional_tag2 ... --raw` : check tags for a GPG signature and print the raw details

## Categories
security, version-control

## Links
- More information: https://git-scm.com/docs/git-verify-tag
- tldr page: https://tldr.inbrowser.app/pages/common/git-verify-tag
- Wikipedia: https://en.wikipedia.org/wiki/Git
