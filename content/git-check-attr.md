# git check-attr

For every pathname, list if each attribute is unspecified, set, or unset as a gitattribute on that pathname.

## Usage
- `git check-attr -a path/to/file` : check the values of all attributes on a file
- `git check-attr attribute path/to/file` : check the value of a specific attribute on a file
- `git check-attr -a path/to/file1 path/to/file2 ...` : check the values of all attributes on specific files
- `git check-attr attribute path/to/file1 path/to/file2 ...` : check the value of a specific attribute on one or more files

## Categories
files, version-control

## Links
- More information: https://git-scm.com/docs/git-check-attr
- tldr page: https://tldr.inbrowser.app/pages/common/git-check-attr
