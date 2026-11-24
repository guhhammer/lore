# pass

Store and read passwords or other sensitive data. All data is GPG-encrypted, and managed with a Git repository.

## Usage
- `pass init gpg_id_1 gpg_id_2 ...` : initialize (or re-encrypt) the storage using one or more GPG IDs
- `pass insert -m path/to/data` : save a new password and additional information (press `<Ctrl d>` on a new line to complete)
- `pass edit path/to/data` : edit an entry
- `pass -c path/to/data` : copy a password (first line of the data file) to the clipboard
- `pass` : list the whole store tree
- `pass generate -c path/to/data num` : generate a new random password with a given length, and copy it to the clipboard
- `pass git init` : initialize a new Git repository (any changes done by pass will be committed automatically)
- `pass git command` : run a Git command on behalf of the password storage

## Categories
version-control, files, security

## Links
- More information: https://git.zx2c4.com/password-store/about/
- tldr page: https://tldr.inbrowser.app/pages/common/pass
- Wikipedia: https://en.wikipedia.org/wiki/Pass_(software)
