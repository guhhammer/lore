# passage

Store and read passwords or other sensitive data. Note: All data is age-encrypted (as opposed to GPG used by `pass`).

## Usage
- `passage insert -m path/to/data` : insert a new password (press `<Ctrl d>` on a new line to complete)
- `echo "password" | passage insert -e path/to/data` : insert a new password from `stdin`
- `passage path/to/data` : show a password
- `passage -c path/to/data` : copy a password to the clipboard
- `passage ls` : list the whole store tree
- `passage generate path/to/data length` : generate a new random password of a given length
- `passage edit path/to/data` : edit an entry
- `passage rm path/to/data` : remove an entry

## See also
- pass

## Categories
files, users, security

## Links
- More information: https://github.com/FiloSottile/passage
- tldr page: https://tldr.inbrowser.app/pages/common/passage
