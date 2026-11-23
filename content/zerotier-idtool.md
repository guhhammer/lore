# zerotier-idtool

Create and manipulate ZeroTier identities.

## Usage
- `zerotier-idtool generate` : generate a new ZeroTier identity and output the secret part to `stdout`
- `zerotier-idtool generate path/to/identity.secret path/to/identity.public` : generate a new ZeroTier identity and save the secret and public parts to files
- `zerotier-idtool generate path/to/identity.secret path/to/identity.public vanity_prefix` : generate a new ZeroTier identity with a specific hexadecimal vanity prefix (can take a long time)
- `zerotier-idtool getpublic path/to/identity.secret` : extract the public portion from a secret identity
- `zerotier-idtool sign path/to/identity.secret path/to/file` : sign a file using a secret identity
- `zerotier-idtool verify path/to/identity.public path/to/file signature_hex` : verify a signed file using a public identity and a hexadecimal signature
- `zerotier-idtool validate path/to/identity.public` : locally validate an identity's key and proof of work
- `zerotier-idtool help` : display help

## See also
- zerotier-cli
- zerotier-one

## Categories
files

## Links
- More information: https://github.com/zerotier/ZeroTierOne/blob/dev/doc/zerotier-idtool.1.md
- tldr page: https://tldr.inbrowser.app/pages/common/zerotier-idtool
