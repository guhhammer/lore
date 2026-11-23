# smbserver.py

Host SMB shares. Part of the Impacket suite.

## Usage
- `smbserver.py sharename path/to/share` : set up a basic SMB share
- `smbserver.py -comment my_share sharename path/to/share` : set up a share with a custom comment
- `smbserver.py -username username -password password sharename path/to/share` : set up a share with username and password authentication
- `smbserver.py -hashes LMHASH:NTHASH sharename path/to/share` : set up a share with NTLM hash authentication
- `smbserver.py -ip interface_ip_address sharename path/to/share` : set up a share on a specific interface
- `smbserver.py -port port sharename path/to/share` : set up a share on a non-standard SMB port
- `smbserver.py -smb2support sharename path/to/share` : set up a share with SMB2 support
- `smbserver.py -outputfile path/to/output_file sharename path/to/share` : set up a share and log commands to an output file

## Categories
files, network, users

## Links
- More information: https://github.com/fortra/impacket
- tldr page: https://tldr.inbrowser.app/pages/common/smbserver.py
