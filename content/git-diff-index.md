# git diff-index

Compare the working directory with a commit or tree object.

## Usage
- `git diff-index commit` : compare the working directory with a specific commit
- `git diff-index commit path/to/file_or_directory` : compare a specific file or directory in working directory with a commit
- `git diff-index --cached commit` : compare the working directory with the index (staging area) to check for staged changes
- `git diff-index --quiet commit` : suppress output and return an exit status to check for differences

## Categories
version-control, files

## Links
- More information: https://git-scm.com/docs/git-diff-index
- tldr page: https://tldr.inbrowser.app/pages/common/git-diff-index
