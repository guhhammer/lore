# fprintd

Fingerprint management daemon. `fprintd` isn't a command, but a set of commands.

## Usage
- `fprintd-list username` : list enrolled fingerprints for a specific user
- `fprintd-list username1 username2 ...` : list enrolled fingerprints for one or more users
- `fprintd-enroll` : enroll the right index finger for the current user
- `fprintd-enroll -f left-thumb|left-index-finger|left-middle-finger|left-ring-finger|left-little-finger|right-thumb|...` : enroll a specific finger for the current user
- `fprintd-verify` : verify all stored fingerprints for the current user
- `fprintd-verify -f left-thumb|left-index-finger|left-middle-finger|left-ring-finger|left-little-finger|right-thumb|...` : verify a specific fingerprint for the current user
- `fprintd-delete username` : remove all fingerprints for a specific user
- `fprintd-delete username -f left-thumb|left-index-finger|left-middle-finger|left-ring-finger|left-little-finger|right-thumb|...` : remove a specific fingerprints for a specific user

## Categories
users, system

## Links
- More information: https://fprint.freedesktop.org/
- tldr page: https://tldr.inbrowser.app/pages/linux/fprintd
