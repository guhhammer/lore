# samba-tool

Main Samba administration tool.

## Usage
- `samba-tool [-h] [-W myworkgroup] [-U user] [-d debuglevel] [--v]` : the general form, as the manual gives it
- `samba-tool --simple-bind-dn=DN` : dN to use for a simple bind
- `samba-tool --password` : specify the password on the commandline
- `samba-tool --use-krb5-ccache=CCACHE` : specifies the credential cache location for Kerberos authentication
- `samba-tool --ipaddress=IPADDRESS` : iP address of the server

## Categories
users
