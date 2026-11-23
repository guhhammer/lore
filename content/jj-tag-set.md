# jj tag set

Create or update tags in a `jj` repository.

## Usage
- `jj tag s tag_name` : create a tag pointing to the current working copy revision
- `jj tag s tag_name -r revision` : create a tag pointing to a specific revision
- `jj tag s tag1 tag2 ... -r revision` : create multiple tags pointing to the same revision
- `jj tag s tag_name -r revision --allow-move` : move an existing tag to a different revision
- `jj tag s tag_name -r @-` : create a tag pointing to the parent of the current revision

## See also
- jj tag delete
- jj tag list

## Categories
files, packages

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-tag-set
- tldr page: https://tldr.inbrowser.app/pages/common/jj-tag-set
