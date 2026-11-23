# in-toto-sign

Sign in-toto link or layout metadata or verify their signatures.

## Usage
- `in-toto-sign -f unsigned.layout -k priv_key1 priv_key2 -o root.layout` : sign 'unsigned.layout' with two keys and write it to 'root.layout'
- `in-toto-sign -f package.2f89b927.link -k priv_key` : replace signature in link file and write to default filename
- `in-toto-sign -f root.layout -k pub_key0 pub_key1 pub_key2 --verify` : verify a layout signed with 3 keys
- `in-toto-sign -f root.layout -g` : sign a layout with the default GPG key in default GPG keyring
- `in-toto-sign -f root.layout --verify -g ...439F3C2` : verify a layout with a GPG key identified by keyid '...439F3C2'

## Categories
files, security

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/in-toto-sign
