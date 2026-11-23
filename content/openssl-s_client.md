# openssl s_client

Create TLS client connections.

## Usage
- `openssl s_client -connect host:port 2>/dev/null | openssl x509 -noout -dates` : display the start and expiry dates for a domain's certificate
- `openssl </dev/null s_client -connect host:port` : display the certificate presented by an SSL/TLS server
- `openssl s_client -connect host:port -servername hostname` : set the Server Name Indicator (SNI) when connecting to the SSL/TLS server
- `openssl </dev/null s_client -connect host:443 -showcerts` : display the complete certificate chain of an HTTPS server

## Categories
network, security, desktop

## Links
- More information: https://docs.openssl.org/master/man1/openssl-s_client/
- tldr page: https://tldr.inbrowser.app/pages/common/openssl-s_client
