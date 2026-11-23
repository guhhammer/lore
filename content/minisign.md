# minisign

A dead simple tool to sign files and verify signatures.

## Usage
- `minisign -G` : generate a new keypair at the default location
- `minisign -Sm path/to/file` : sign a file
- `minisign -Sm path/to/file -c "Untrusted comment" -t "Trusted comment"` : sign a file, adding a trusted (signed) and an untrusted (unsigned) comment in the signature
- `minisign -Vm path/to/file -p path/to/publickey.pub` : verify a file and the trusted comments in its signature using the specified public key file
- `minisign -Vm path/to/file -P "public_key_base64"` : verify a file and the trusted comments in its signature, specifying a public key as a Base64 encoded literal

## Categories
files, security

## Links
- More information: https://jedisct1.github.io/minisign/
- tldr page: https://tldr.inbrowser.app/pages/common/minisign
