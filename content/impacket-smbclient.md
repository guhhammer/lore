# impacket-smbclient

This command is an alias of `smbclient.py`.

## Usage
- `smbclient.py domain/username:password@target` : connect to an SMB server with username and password
- `smbclient.py -hashes LM_HASH:NT_HASH domain/username@target` : connect using NTLM hashes for authentication
- `smbclient.py -k domain/username@target` : connect using Kerberos authentication
- `smbclient.py -dc-ip domain_controller_ip domain/username:password@target` : connect specifying a domain controller IP
- `smbclient.py -target-ip target_ip domain/username:password@target` : connect to a specific target IP instead of NetBIOS name
- `smbclient.py -port port domain/username:password@target` : connect to a non-standard SMB port
- `smbclient.py -inputfile path/to/input_file domain/username:password@target` : execute commands from an input file in the SMB shell
- `smbclient.py -outputfile path/to/output_file domain/username:password@target` : log SMB client commands to an output file

## Categories
users, files, network

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/impacket-smbclient
