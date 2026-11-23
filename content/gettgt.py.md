# gettgt.py

Request a Ticket Granting Ticket (TGT). Part of the Impacket suite.

## Usage
- `getTGT.py domain/username:password` : request a TGT using a password
- `getTGT.py -hashes LM_Hash:NT_Hash domain/username` : request a TGT using NTLM hashes
- `getTGT.py -k -no-pass domain/username` : use Kerberos authentication (from existing ccache, no password needed)
- `getTGT.py -aesKey aes_key domain/username` : request a TGT using an AES key (128 or 256 bits)
- `getTGT.py -dc-ip domain_controller_ip domain/username:password` : specify a domain controller IP
- `getTGT.py -service SPN domain/username:password` : request a service ticket directly (AS-REQ) for a specific SPN

## Categories
users

## Links
- More information: https://github.com/fortra/impacket
- tldr page: https://tldr.inbrowser.app/pages/common/gettgt.py
