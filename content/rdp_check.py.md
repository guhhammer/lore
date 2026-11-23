# rdp_check.py

Test whether an account is valid on the target host using the RDP protocol (no full login, just authentication check). Part of the Impacket suite.

## Usage
- `rdp_check.py domain/username@target` : check if credentials are valid on a target (password prompted if omitted)
- `rdp_check.py -hashes LM_Hash:NT_Hash domain/username@target` : check credentials using NTLM hashes
- `rdp_check.py domain/username:password@target` : check credentials with explicit password
- `rdp_check.py username:password@target` : check credentials for a local account on the target (no domain)

## Categories
users, network, development

## Links
- More information: https://github.com/fortra/impacket
- tldr page: https://tldr.inbrowser.app/pages/common/rdp_check.py
