# jj tag

Manage tags in a `jj` repository. Some subcommands such as `delete`, `list`, and `set` have their own usage documentation.

## Usage
- `jj tag s tag_name` : create a tag pointing to the current working copy revision
- `jj tag s tag_name -r revision` : create a tag pointing to a specific revision
- `jj tag l` : list all tags
- `jj tag l --sort committer-date- "pattern"` : list tags matching a pattern, sorted by committer date (newest first)
- `jj tag s tag_name -r revision --allow-move` : move an existing tag to a different revision
- `jj tag d tag_name` : delete a tag
- `jj tag d "glob:v1.*"` : delete tags matching a `glob` pattern
- `jj tag -h` : display help

## Categories
documents, files, monitoring

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-tag
- tldr page: https://tldr.inbrowser.app/pages/common/jj-tag
