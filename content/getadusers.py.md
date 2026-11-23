# getadusers.py

Retrieve a list of users from Active Directory, including attributes like last logon timestamp and email. Part of the Impacket suite.

## Usage
- `GetADUsers.py -all -dc-ip domain_controller_ip domain/username:password` : enumerate all Active Directory users and their attributes
- `GetADUsers.py -user user -dc-ip domain_controller_ip domain/username:password` : retrieve information only for a specific user
- `GetADUsers.py -all -dc-ip domain_controller_ip -hashes LM_Hash:NT_Hash domain/username` : extract user details using pass-the-hash authentication
- `GetADUsers.py -all -dc-ip domain_controller_ip domain/username:password >path/to/output.txt` : save output to a file

## Categories
users, files

## Links
- More information: https://github.com/fortra/impacket
- tldr page: https://tldr.inbrowser.app/pages/common/getadusers.py
