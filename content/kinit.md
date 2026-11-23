# kinit

Authenticate a principal with a Kerberos server to gain and cache a ticket. Note: A Kerberos principal can be either a user, service, or application.

## Usage
- `kinit username` : authenticate a user and obtain a ticket-granting ticket
- `kinit -R` : renew a ticket-granting ticket
- `kinit -l 5h` : specify a lifetime for the ticket
- `kinit -r 1w` : specify a total renewable lifetime for the ticket
- `kinit -p principal@REALM` : specify a different principal name to authenticate as
- `kinit -t path/to/keytab` : specify a different keytab file to authenticate with

## Categories
users, system

## Links
- More information: https://web.mit.edu/kerberos/krb5-latest/doc/user/user_commands/kinit.html
- tldr page: https://tldr.inbrowser.app/pages/common/kinit
