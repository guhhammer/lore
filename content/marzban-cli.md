# marzban cli

Manage administrators, users, and subscriptions.

## Usage
- `marzban cli admin list` : list all administrators
- `marzban cli admin create -u username` : create a new administrator
- `marzban cli user list` : list all users
- `marzban cli user set-owner -u username --owner owner` : transfer ownership of a user to another administrator
- `marzban cli subscription get-config -u username -f v2ray|clash` : generate a subscription configuration for a user
- `marzban cli subscription get-link -u username` : display a user's subscription link
- `marzban cli --help` : display help

## Categories
users

## Links
- More information: https://github.com/Gozargah/Marzban/blob/master/cli/README.md
- tldr page: https://tldr.inbrowser.app/pages/linux/marzban-cli
