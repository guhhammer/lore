# gitlint

Git commit message linter checks your commit messages for style.

## Usage
- `gitlint` : check the last commit message
- `gitlint --commits single_refspec_argument` : lint a specific range of commits
- `gitlint --extra-path path/to/directory` : use a directory or Python module with extra user-defined rules
- `gitlint --target path/to/target_directory` : start a specific CI job
- `gitlint --msg-filename path/to/file` : lint a commit message from a specific file
- `gitlint --staged` : read staged commit meta-info from the local repository

## Categories
version-control, files

## Links
- More information: https://jorisroovers.com/gitlint/
- tldr page: https://tldr.inbrowser.app/pages/common/gitlint
