# getnpusers.py

Enumerate Active Directory accounts with Kerberos pre-authentication disabled, which may be susceptible to AS-REP roasting attacks. Part of the Impacket suite.

## Usage
- `GetNPUsers.py domain/ -usersfile path/to/userslist -dc-ip domain_controller_ip -no-pass` : enumerate users with Kerberos pre-authentication disabled (default anonymous enumeration)
- `GetNPUsers.py domain/ -usersfile path/to/userslist -dc-ip domain_controller_ip -no-pass -request` : perform AS-REP roasting and dump crackable hashes for offline cracking
- `GetNPUsers.py domain/username:password -usersfile path/to/userslist -dc-ip domain_controller_ip` : authenticate with valid credentials (if anonymous binding is disabled)
- `GetNPUsers.py domain/username -hashes LM_Hash:NT_Hash -usersfile path/to/userslist -dc-ip domain_controller_ip` : use pass-the-hash authentication instead of a password
- `GetNPUsers.py domain/ -usersfile path/to/userslist -dc-ip domain_controller_ip -request >path/to/output.txt` : save the output to a file for further analysis

## Categories
files, users

## Links
- More information: https://github.com/fortra/impacket
- tldr page: https://tldr.inbrowser.app/pages/common/getnpusers.py
