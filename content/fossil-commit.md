# fossil commit

Commit files to a Fossil repository.

## Usage
- `fossil ci` : create a new version containing all the changes in the current checkout; user will be prompted for a comment
- `fossil ci -m "comment"` : create a new version containing all the changes in the current checkout, using the specified message
- `fossil ci -M path/to/commit_message_file` : create a new version containing all the changes in the current checkout with a comment read from a specific file
- `fossil ci path/to/file1 path/to/file2 ...` : create a new version containing changes from the specified files; user will be prompted for a comment

## Categories
version-control, files, packages

## Links
- More information: https://fossil-scm.org/home/help/commit
- tldr page: https://tldr.inbrowser.app/pages/common/fossil-commit
- Wikipedia: https://en.wikipedia.org/wiki/Fossil_(software)
