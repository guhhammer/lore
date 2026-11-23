# kcadm.sh

Perform administration tasks.

## Usage
- `kcadm.sh config credentials --server host --realm realm_name --user username --password password` : start an authenticated session
- `kcadm.sh create users -s username=username -r realm_name` : create a user
- `kcadm.sh get realms` : list all realms
- `kcadm.sh update realms/realm_name -f path/to/file.json` : update a realm with JSON config

## Categories
users, network

## Links
- More information: https://www.keycloak.org/docs/latest/server_admin/#admin-cli
- tldr page: https://tldr.inbrowser.app/pages/common/kcadm.sh
