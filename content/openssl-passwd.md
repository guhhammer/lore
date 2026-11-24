# openssl passwd

Generate password hash.

## Usage
- `openssl passwd -5` : generate password hash in SHA256 algorithm (Linux only)
- `openssl passwd -apr1` : generate password hash in APR1 algorithm
- `openssl passwd -apr1 -salt salt_string` : generate password hash in APR1 algorithm with salt
- `openssl passwd -apr1 -table` : generate password hash in APR1 and display the password and the hash
- `echo -n "password" | openssl passwd -apr1 -stdin` : generate password hash from `stdin`

## Categories
users, security

## Links
- More information: https://docs.openssl.org/master/man1/openssl-passwd/
- tldr page: https://tldr.inbrowser.app/pages/common/openssl-passwd
- Wikipedia: https://en.wikipedia.org/wiki/OpenSSL
