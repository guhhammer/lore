# getuserspns.py

Retrieve Service Principal Names (SPNs) associated with Active Directory user accounts. Part of the Impacket suite.

## Usage
- `GetUserSPNs.py domain/username:password -dc-ip domain_controller_ip` : enumerate user accounts with an SPN and request their Kerberos TGS tickets
- `GetUserSPNs.py domain/username -hashes LM_Hash:NT_Hash -dc-ip domain_controller_ip` : use pass-the-hash authentication
- `GetUserSPNs.py domain/username:password -dc-ip domain_controller_ip -outputfile path/to/output_file` : save the output to a file
- `GetUserSPNs.py domain/username:password -dc-ip domain_controller_ip -request` : request only TGS tickets
- `GetUserSPNs.py domain/username -dc-ip domain_controller_ip -hashes LM_Hash:NT_Hash -request` : request only TGS tickets using pass-the-hash authentication

## Categories
users, files, system

## Links
- More information: https://github.com/fortra/impacket
- tldr page: https://tldr.inbrowser.app/pages/common/getuserspns.py
