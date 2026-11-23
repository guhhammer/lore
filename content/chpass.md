# chpass

Add or change user database information, including login shell and password. Note: It's not possible to change the user's password on Open Directory systems, use `passwd` instead.

## Usage
- `su -c chpass` : add or change user database information for the current user interactively
- `chpass -s path/to/shell` : set a specific login shell for the current user
- `chpass -s path/to/shell username` : set a login shell for a specific user
- `chpass -l location -s path/to/shell username` : edit the user record on the directory node at the given location
- `chpass -u authname -s path/to/shell username` : use the given username when authenticating to the directory node containing the user

## See also
- passwd

## Categories
users, files, shell, macos

## Links
- More information: https://man.freebsd.org/cgi/man.cgi?query=chpass
- tldr page: https://tldr.inbrowser.app/pages/osx/chpass
