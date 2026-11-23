# salt-key

Manage salt minion keys on the salt master. Needs to be run on the salt master, likely as root or with `sudo`.

## Usage
- `salt-key -L` : list all accepted, unaccepted, and rejected minion keys
- `salt-key -a minion_id` : accept a minion key by name
- `salt-key -r minion_id` : reject a minion key by name
- `salt-key -F` : print fingerprints of all public keys

## Categories
users

## Links
- More information: https://docs.saltproject.io/en/latest/ref/cli/salt-key.html
- tldr page: https://tldr.inbrowser.app/pages/common/salt-key
