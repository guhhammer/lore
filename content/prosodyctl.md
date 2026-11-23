# prosodyctl

The control tool for the Prosody XMPP server. Note: Process management through `prosodyctl` is discouraged. Instead, use the tools provided by your system (e.g. `systemctl`).

## Usage
- `sudo prosodyctl status` : show the status of the Prosody server
- `sudo prosodyctl reload` : reload the server's configuration files
- `sudo prosodyctl adduser user@example.com` : add a user to the Prosody XMPP server
- `sudo prosodyctl passwd user@example.com` : set a user's password
- `sudo prosodyctl deluser user@example.com` : permanently delete a user

## Categories
users, processes

## Links
- More information: https://prosody.im/doc/prosodyctl
- tldr page: https://tldr.inbrowser.app/pages/common/prosodyctl
