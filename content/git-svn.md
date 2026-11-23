# git svn

Bidirectional operation between a Subversion repository and Git.

## Usage
- `git svn clone https://example.com/subversion_repo path/to/local_directory` : clone an SVN repository
- `git svn clone -r 1234:HEAD https://svn.example.net/subversion/repo path/to/local_directory` : clone an SVN repository starting at a given revision number
- `git svn rebase` : update local clone from the remote SVN repository
- `git svn fetch` : fetch updates from the remote SVN repository without changing the Git `HEAD`
- `git svn commit` : commit back to the SVN repository

## Categories
version-control, packages, network

## Links
- More information: https://git-scm.com/docs/git-svn
- tldr page: https://tldr.inbrowser.app/pages/common/git-svn
