# apptainer config

Manage various Apptainer configurations.

## Usage
- `sudo apptainer config fakeroot -a username` : add a fakeroot user mapping
- `sudo apptainer config fakeroot -r username` : remove a fakeroot user mapping
- `sudo apptainer config fakeroot -e username` : enable a fakeroot user mapping
- `sudo apptainer config fakeroot -d username` : disable a fakeroot user mapping
- `sudo apptainer config global -g directive` : get the current value of a global configuration directive
- `sudo apptainer config global -s directive value` : set a global configuration directive value
- `sudo apptainer config global -u directive value` : unset a global configuration directive value
- `sudo apptainer config global -r directive` : reset a global configuration directive to its default value

## Categories
users

## Links
- More information: https://apptainer.org/docs/user/main/cli/apptainer_config.html
- tldr page: https://tldr.inbrowser.app/pages/linux/apptainer-config
- Wikipedia: https://en.wikipedia.org/wiki/Apptainer
