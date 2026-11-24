# openssl

openssl makes keys and certificates, reads them, checks what a server presents and converts between formats.

## Tasks
### Make keys and certificates
- `openssl genrsa -out key.pem 4096` : make a private RSA key
- `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj "/CN=localhost"` : a self-signed certificate for testing
- `openssl req -new -key key.pem -out request.csr -subj "/CN=example.org"` : a signing request to send to a CA
- `openssl rand -hex 32` : 32 random bytes as hex
- `openssl rand -base64 24` : a random password-like string
### Read them
- `openssl x509 -in cert.pem -noout -text` : every field of a certificate
- `openssl x509 -in cert.pem -noout -subject -issuer -dates` : who, by whom and when it expires
- `openssl x509 -in cert.pem -noout -fingerprint -sha256` : the SHA-256 fingerprint
- `openssl req -in request.csr -noout -text` : read a signing request
- `openssl x509 -in cert.pem -noout -ext subjectAltName` : the names it covers
### Check a server
- `openssl s_client -connect example.org:443 -servername example.org` : open a TLS connection and print the certificate
- `echo | openssl s_client -connect example.org:443 -servername example.org 2>/dev/null | openssl x509 -noout -dates` : just the expiry dates
- `openssl s_client -connect example.org:443 -showcerts` : show the whole chain
- `openssl s_client -connect HOST:587 -starttls smtp` : test a mail server's STARTTLS
- `openssl verify -CAfile ca.pem cert.pem` : check a certificate against a CA file
### Match and convert
- `openssl x509 -in cert.pem -noout -pubkey | openssl sha256` : hash of a certificate's public key
- `openssl pkey -in key.pem -pubout | openssl sha256` : the same from a private key; equal output means they match
- `openssl pkcs12 -export -out bundle.p12 -inkey key.pem -in cert.pem` : make a .p12 file
- `openssl pkcs12 -in bundle.p12 -out all.pem -nodes` : unpack a .p12 into PEM (the key is unencrypted)
- `openssl x509 -in cert.der -inform der -out cert.pem` : DER to PEM
- `openssl x509 -in cert.pem -outform der -out cert.der` : PEM to DER
### Files
- `openssl enc -aes-256-cbc -pbkdf2 -in FILE -out FILE.enc` : encrypt a file with a passphrase
- `openssl dgst -sha256 FILE` : a file's SHA-256 hash

## Notes
- Keep the private key private: mode 600, never in git.
- `-nodes` writes a key without a passphrase; fine for tests, not for sharing.
- Give `-servername` when testing a site on shared hosting, or you get the wrong certificate.
- Browsers need the names in subjectAltName, not just the common name.
- For a real site use Let's Encrypt (see certbot) instead of a self-signed certificate.

## See also
- certbot
- gpg
- ssh-keys
- curl

## Links
- Manual: https://docs.openssl.org/master/man1/openssl/
- Commands: https://docs.openssl.org/master/man1/

## Categories
security network
