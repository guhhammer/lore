# lpass

Interface for the LastPass password manager.

## Usage
- `lpass login username` : log in to your LastPass account, by entering your master password when prompted
- `lpass status` : show login status
- `lpass ls` : list all sites grouped by category
- `lpass generate --username username --url gmail.com myinbox password_length` : generate a new password for gmail.com with the identifier `myinbox` and add to LastPass
- `lpass show myinbox --password` : show password for a specified entry

## Categories
users, network, security

## Links
- More information: https://lastpass.github.io/lastpass-cli/lpass.1.html
- tldr page: https://tldr.inbrowser.app/pages/common/lpass
