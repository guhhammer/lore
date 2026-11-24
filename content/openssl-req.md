# openssl req

Manage PKCS#10 Certificate Signing Requests.

## Usage
- `openssl req -new -sha256 -key filename.key -out filename.csr` : generate a certificate signing request to be sent to a certificate authority
- `openssl req -new -x509 -newkey rsa:4096 -keyout filename.key -out filename.cert -subj "/C=XX/CN=foobar" -days 365` : generate a self-signed certificate and a corresponding key-pair, storing both in a file

## Categories
security

## Links
- More information: https://docs.openssl.org/master/man1/openssl-req/
- tldr page: https://tldr.inbrowser.app/pages/common/openssl-req
- Wikipedia: https://en.wikipedia.org/wiki/OpenSSL
