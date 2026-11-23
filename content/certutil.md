# certutil

Manage keys and certificates in both NSS databases and other NSS tokens.

## Usage
- `certutil -N -d .` : create a New certificate database in the current directory
- `certutil -L -d .` : list all certificates in a database
- `certutil -K -d . -f path/to/password_file.txt` : list all private Keys in a database specifying the password file
- `certutil -A -n "server_certificate" -t ",," -i path/to/file.crt -d .` : add the signed certificate to the requesters database specifying a nickname, trust attributes and an input CRT file
- `certutil -S -f path/to/password_file.txt -d . -t ",," -c "server_certificate" -n "server_name" -g 2048 -s "CN=common_name,O=organization"` : add subject alternative names to a given certificate with a specific key size (g)

## Categories
files, databases, security

## Links
- More information: https://manned.org/certutil
- tldr page: https://tldr.inbrowser.app/pages/common/certutil
