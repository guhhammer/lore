# jj tag list

List tags in a `jj` repository.

## Usage
- `jj tag l` : list all tags
- `jj tag l "pattern"` : list tags matching a pattern
- `jj tag l "substring:release"` : list tags matching a substring pattern
- `jj tag l --sort committer-date-` : list tags sorted by committer date (newest first)
- `jj tag l --sort name-` : list tags sorted by name in descending order
- `jj tag l -T "template"` : list tags with a custom template

## See also
- jj tag delete
- jj tag set

## Categories
packages, text

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-tag-list
- tldr page: https://tldr.inbrowser.app/pages/common/jj-tag-list
