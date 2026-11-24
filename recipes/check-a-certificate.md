# check a certificate

See a site's TLS certificate, when it expires and who issued it, or the same for a .pem file.

## Steps
- `openssl s_client -connect example.com:443 -servername example.com < /dev/null | openssl x509 -noout -dates -subject -issuer` : a site's certificate: dates, name, issuer
- `openssl x509 -in cert.pem -noout -enddate` : when a certificate file expires
- `openssl x509 -in cert.pem -noout -text` : everything in it
- `openssl x509 -in cert.pem -noout -checkend 2592000` : does it expire within 30 days? (exit 1 if so)
- `curl -vI https://example.com 2>&1 | grep -E 'expire|issuer'` : the same, the quick way

## See also
- openssl
- curl

## Categories
security network
