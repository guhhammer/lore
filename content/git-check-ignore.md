# git check-ignore

Analyze and debug Git ignore/exclude (`.gitignore`) files.

## Usage
- `git check-ignore path/to/file_or_directory` : check whether a file or directory is ignored
- `git check-ignore path/to/file_or_directory1 path/to/file_or_directory2 ...` : check whether multiple files or directories are ignored
- `git <path/to/file_list check-ignore --stdin` : use pathnames, one per line, from `stdin`
- `git check-ignore --no-index path/to/file_or_directory1 path/to/file_or_directory2 ...` : do not check the index (used to debug why paths were tracked and not ignored)
- `git check-ignore -v path/to/file_or_directory1 path/to/file_or_directory2 ...` : include details about the matching pattern for each path

## Categories
version-control, files, development

## Links
- More information: https://git-scm.com/docs/git-check-ignore
- tldr page: https://tldr.inbrowser.app/pages/common/git-check-ignore
- Wikipedia: https://en.wikipedia.org/wiki/Git
