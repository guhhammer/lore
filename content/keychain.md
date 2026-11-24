# keychain

Re-use ssh-agent and/or gpg-agent between logins.

## Usage
- `keychain` : check for a running ssh-agent, and start one if needed
- `keychain --agents "gpg,ssh"` : also check for gpg-agent
- `keychain --list` : list signatures of all active keys
- `keychain --list-fp` : list fingerprints of all active keys
- `keychain --timeout minutes` : add a timeout for identities added to the agent, in minutes

## Categories
network, security

## Links
- More information: https://www.funtoo.org/Keychain
- tldr page: https://tldr.inbrowser.app/pages/common/keychain
- Wikipedia: https://en.wikipedia.org/wiki/Keychain_(software)
