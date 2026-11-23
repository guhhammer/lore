# apt-key

Key management utility for the APT Package Manager on Debian and Ubuntu. Note: This command is deprecated (except for the use of `apt-key del` in maintainer scripts).

## Usage
- `apt-key list` : list trusted keys
- `apt-key add public_key_file.asc` : add a key to the trusted keystore
- `apt-key del key_id` : delete a key from the trusted keystore
- `wget -qO - https://host.tld/filename.key | apt-key add -` : add a remote key to the trusted keystore
- `apt-key adv --keyserver pgp.mit.edu --recv key_id` : add a key from keyserver with only key ID

## Categories
network, packages

## Links
- More information: https://manned.org/apt-key
- tldr page: https://tldr.inbrowser.app/pages/linux/apt-key
