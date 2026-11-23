# jj tag delete

Delete tags in a `jj` repository.

## Usage
- `jj tag d tag_name` : delete a tag
- `jj tag d tag1 tag2 ...` : delete multiple tags
- `jj tag d "glob:v1.*"` : delete tags matching a `glob` pattern
- `jj tag d "substring:release"` : delete tags matching a substring pattern
- `jj tag d "exact:v1.0.0"` : delete a tag by exact name

## See also
- jj tag list
- jj tag set

## Categories
packages

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-tag-delete
- tldr page: https://tldr.inbrowser.app/pages/common/jj-tag-delete
