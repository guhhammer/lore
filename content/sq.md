# sq

A modern OpenPGP command-line tool.

## Usage
- `sq encrypt --with-password --without-signature path/to/file --output path/to/file.pgp` : encrypt a file using a password (symmetric encryption)
- `sq decrypt path/to/file.pgp --output path/to/file` : decrypt a password-protected file
- `sq inspect path/to/file.pgp` : inspect an OpenPGP file to see its metadata and structure
- `sq verify --signer-file path/to/signer.asc --signature-file path/to/file.sig path/to/file` : verify a file with a detached signature and a certificate file
- `sq verify --signer-file path/to/signer.asc --cleartext path/to/file` : verify a file with an embedded (cleartext) signature and a certificate file
- `sq key generate --own-key --name name --email name@example.com` : generate own key, and save it on the local key store
- `sq key|cert list` : list all secret keys or certs managed by the local key store
- `sq config inspect paths` : list current configuration and storage paths

## See also
- gpg

## Categories
files, security, text

## Links
- More information: https://sequoia-pgp.gitlab.io/sequoia-sq/man/sq.1.html
- tldr page: https://tldr.inbrowser.app/pages/common/sq
