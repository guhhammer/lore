# testssl

Check SSL/TLS protocols and ciphers supported by a server.

## Usage
- `testssl example.com` : test a server (run every check) on port 443
- `testssl example.com:465` : test a different port
- `testssl --protocols example.com` : only check available protocols
- `testssl --vulnerable example.com` : only check vulnerabilities
- `testssl --headers example.com` : only check HTTP security headers
- `testssl --starttls ftp|smtp|pop3|imap|xmpp|sieve|xmpp-server|telnet|ldap|irc|lmtp|nntp|postgres|mysql example.com:port` : test other STARTTLS enabled protocols

## Categories
network, security, development

## Links
- More information: https://testssl.sh/doc/testssl.1.html
- tldr page: https://tldr.inbrowser.app/pages/common/testssl
