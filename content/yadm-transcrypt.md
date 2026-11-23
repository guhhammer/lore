# yadm transcrypt

If `transcrypt` is installed, allows you to pass options directly to `transcrypt`. With the environment configured to use the yadm repository. Transcrypt enables transparent encryption and decryption of files in a Git repository.

## Usage
- `yadm transcrypt -c cipher` : set the symmetric cipher to utilize for encryption
- `yadm transcrypt -p password` : pass the password to derive the key from
- `yadm transcrypt -y` : assume yes and accept defaults for non-specified options
- `yadm transcrypt -d` : display the current repository's cipher and password
- `yadm transcrypt -r` : re-encrypt all encrypted files using new credentials

## Categories
packages, users, version-control

## Links
- More information: https://github.com/elasticdog/transcrypt#command-line-options
- tldr page: https://tldr.inbrowser.app/pages/common/yadm-transcrypt
