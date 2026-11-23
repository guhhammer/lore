# openssl rand

Generate random bytes using a cryptographically secure PRNG.

## Usage
- `openssl rand -hex 8` : generate an 8-byte (16 characters) hex string and write it to `stdout`
- `openssl rand -base64 20` : generate 20 random bytes encoded in base64
- `openssl rand -out path/to/file length` : generate random bytes and write them to a file (without encoding)
- `openssl rand -hex|base64 1K|M|G|T` : generate 1 KiB/MiB/GiB/TiB random bytes encoded in hex/base64

## Categories
files, security

## Links
- More information: https://docs.openssl.org/master/man1/openssl-rand/
- tldr page: https://tldr.inbrowser.app/pages/common/openssl-rand
