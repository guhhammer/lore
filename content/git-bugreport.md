# git bugreport

Capture debug information from the system and user, generating a text file to aid in the reporting of a bug in Git.

## Usage
- `git bugreport` : create a new bug report file in the current directory
- `git bugreport -o path/to/directory` : create a new bug report file in the specified directory, creating it if it does not exist
- `git bugreport -s %m%d%y` : create a new bug report file with the specified filename suffix in `strftime` format

## Categories
version-control, files, development

## Links
- More information: https://git-scm.com/docs/git-bugreport
- tldr page: https://tldr.inbrowser.app/pages/common/git-bugreport
