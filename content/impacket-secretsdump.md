# impacket-secretsdump

This command is an alias of `secretsdump.py`.

## Usage
- `secretsdump.py domain/username:password@target` : dump credentials from a Windows machine using a username and password
- `secretsdump.py -hashes LM_Hash:NT_Hash domain/username@target` : dump hashes from a machine using pass-the-hash authentication
- `secretsdump.py -just-dc domain/username:password@target` : dump credentials from Active Directory's NTDS.dit file
- `secretsdump.py -sam path/to/SAM -system path/to/SYSTEM` : extract credentials from a local SAM database using registry hives
- `secretsdump.py -no-pass domain/username@target` : dump hashes from a machine without providing a password (if a valid authentication session exists, e.g. via Kerberos or NTLM SSO)

## Categories
files, users, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/impacket-secretsdump
