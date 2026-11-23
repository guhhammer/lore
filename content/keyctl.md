# keyctl

Manipulate the Linux kernel keyring.

## Usage
- `keyctl list target_keyring` : list keys in a specific keyring
- `keyctl list @us` : list current keys in the user default session
- `keyctl add type_keyring key_name key_value target_keyring` : store a key in a specific keyring
- `echo -n key_value | keyctl padd type_keyring key_name target_keyring` : store a key with its value from `stdin`
- `keyctl timeout key_name timeout_in_seconds` : put a timeout on a key
- `keyctl read key_name` : read a key and format it as a hex-dump if not printable
- `keyctl pipe key_name` : read a key and format as-is
- `keyctl revoke key_name` : revoke a key and prevent any further action on it

## Categories
system

## Links
- More information: https://manned.org/keyctl
- tldr page: https://tldr.inbrowser.app/pages/linux/keyctl
