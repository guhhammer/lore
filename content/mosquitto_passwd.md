# mosquitto_passwd

Manage password files for mosquitto.

## Usage
- `mosquitto_passwd path/to/password_file username` : add a new user to a password file (will prompt to enter the password)
- `mosquitto_passwd -c path/to/password_file username` : create the password file if it doesn't already exist
- `mosquitto_passwd -D path/to/password_file username` : delete the specified username instead
- `mosquitto_passwd -U path/to/password_file` : upgrade an old plain-text password file to a hashed password file

## See also
- mosquitto

## Categories
users, files

## Links
- More information: https://mosquitto.org/man/mosquitto_passwd-1.html
- tldr page: https://tldr.inbrowser.app/pages/common/mosquitto_passwd
