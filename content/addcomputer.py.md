# addcomputer.py

Add a computer account to a domain. Part of the Impacket suite.

## Usage
- `addcomputer.py -computer-name COMPUTER_NAME$ -computer-pass computer_password domain/username:password` : add a computer with a specific name and password
- `addcomputer.py -no-add -computer-name COMPUTER_NAME$ -computer-pass computer_password domain/username:password` : only set a new password on an existing computer
- `addcomputer.py -delete -computer-name COMPUTER_NAME$ domain/username:password` : delete an existing computer account
- `addcomputer.py -k -no-pass domain/username@hostname` : add computer using Kerberos authentication
- `addcomputer.py -method LDAPS -port 636 domain/username:password` : add computer via LDAPS (port 636) instead of SAMR (port 445)
- `addcomputer.py -dc-host hostname domain/username:password` : specify exact domain controller when multiple DCs exist

## Categories
users, network

## Links
- More information: https://github.com/fortra/impacket
- tldr page: https://tldr.inbrowser.app/pages/common/addcomputer.py
