# nxc ldap

Pentest and exploit Windows Active Directory Domains via LDAP.

## Usage
- `nxc ldap 192.168.178.2 -u path/to/usernames.txt -p path/to/passwords.txt` : search for valid domain credentials by trying out every combination in the specified lists of usernames and passwords
- `nxc ldap 192.168.178.2 -u username -p password --active-users` : enumerate active domain users
- `nxc ldap 192.168.178.2 -u username -p password --bloodhound -c All` : collect data about the targeted domain and automatically import these data into BloodHound
- `nxc ldap 192.168.178.2 -u username -p '' --asreproast path/to/output.txt` : attempt to collect AS_REP messages for the specified user in order to perform an ASREPRoasting attack
- `nxc ldap 192.168.178.2 -u username -p password --gmsa` : attempt to extract the passwords of group managed service accounts on the domain

## Categories
users, files

## Links
- More information: https://www.netexec.wiki/ldap-protocol/authentication
- tldr page: https://tldr.inbrowser.app/pages/common/nxc-ldap
