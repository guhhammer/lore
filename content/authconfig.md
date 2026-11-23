# authconfig

Configure system authentication resources.

## Usage
- `authconfig --test` : display the current configuration
- `authconfig --update --passalgo=algorithm` : configure the server to use a different password hashing algorithm
- `authconfig --update --enableldapauth` : enable LDAP authentication
- `authconfig --update --disableldapauth` : disable LDAP authentication
- `authconfig --update --enablenis` : enable Network Information Service (NIS)
- `authconfig --update --enablekrb5` : enable Kerberos
- `authconfig --update --enablewinbindauth` : enable Winbind (Active Directory) authentication
- `authconfig --update --enablelocauthorize` : enable local authorization

## Categories
network

## Links
- More information: https://manned.org/authconfig
- tldr page: https://tldr.inbrowser.app/pages/linux/authconfig
