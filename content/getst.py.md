# getst.py

Request a Kerberos Service Ticket (TGS). Part of the Impacket suite.

## Usage
- `getST.py domain/username:password -spn service/target` : request a service ticket for a specific SPN
- `getST.py -hashes LM_Hash:NT_Hash domain/username -spn service/target` : request a ticket using NTLM hashes (pass-the-hash)
- `getST.py -no-pass -k domain/username -spn service/target` : request a ticket using existing Kerberos ccache file
- `getST.py -k -impersonate target_user domain/username -spn service/target` : impersonate another user via S4U2Self (requires delegation rights)
- `getST.py -force-forwardable -k domain/username -spn service/target` : force the ticket to be forwardable (Bronze Bit)

## Categories
system

## Links
- More information: https://github.com/fortra/impacket
- tldr page: https://tldr.inbrowser.app/pages/common/getst.py
