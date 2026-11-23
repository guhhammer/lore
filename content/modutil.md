# modutil

Manage PKCS #11 module information within the NSS security module database.

## Usage
- `modutil -dbdir sql:path/to/nss_database_directory -add "module_label" -libfile path/to/pkcs11_mod.so` : add a PKCS #11 module to an NSS database (e.g. a Firefox profile: `$HOME/.mozilla/firefox/default-release`)
- `modutil -dbdir sql:path/to/nss_database_directory -list` : list PKCS #11 modules in an NSS database

## Categories
databases, development, system

## Links
- More information: https://manned.org/modutil
- tldr page: https://tldr.inbrowser.app/pages/common/modutil
