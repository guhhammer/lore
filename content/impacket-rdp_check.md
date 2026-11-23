# impacket-rdp_check

This command is an alias of `rdp_check.py`.

## Usage
- `rdp_check.py domain/username@target` : check if credentials are valid on a target (password prompted if omitted)
- `rdp_check.py -hashes LM_Hash:NT_Hash domain/username@target` : check credentials using NTLM hashes
- `rdp_check.py domain/username:password@target` : check credentials with explicit password
- `rdp_check.py username:password@target` : check credentials for a local account on the target (no domain)

## Categories
users, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/impacket-rdp_check
