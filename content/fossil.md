# fossil

Distributed version control system with a built-in wiki, bug tracker, and web interface. Some subcommands such as `commit` have their own usage documentation.

## Usage
- `fossil init repository_name.fossil` : initialize a new empty Fossil repository
- `fossil clone remote_url` : create a local copy of a remote repository
- `fossil status` : show an overview of the current repository state
- `fossil add path/to/file` : stage a new file
- `fossil rm path/to/file` : stage a file for removal
- `fossil ci -m "comment"` : check in all staged changes
- `fossil push remote_url` : push changes from the local repository to a remote repository
- `fossil pull remote_url` : pull changes from a remote repository into the local repository

## Categories
files, packages, version-control

## Links
- More information: https://fossil-scm.org/home/help
- tldr page: https://tldr.inbrowser.app/pages/common/fossil
- Wikipedia: https://en.wikipedia.org/wiki/Fossil_(software)
