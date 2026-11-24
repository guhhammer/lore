# git tag

Create, list, delete, or verify tags. A tag is a static reference to a commit.

## Usage
- `git tag` : list all tags
- `git tag tag_name` : create a tag with the given name pointing to the current commit
- `git tag tag_name commit` : create a tag with the given name pointing to a given commit
- `git tag tag_name -m tag_message` : create an annotated tag with the given message
- `git tag -d tag_name` : delete the tag with the given name
- `git fetch -t` : get updated tags from remote
- `git push --tags` : push all tags to a remote
- `git tag --contains commit` : list all tags which contain a given commit (`HEAD` if not specified)

## Categories
version-control

## Links
- More information: https://git-scm.com/docs/git-tag
- tldr page: https://tldr.inbrowser.app/pages/common/git-tag
- Wikipedia: https://en.wikipedia.org/wiki/Git
