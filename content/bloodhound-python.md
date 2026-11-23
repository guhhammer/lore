# bloodhound-python

A Python ingestor for BloodHound, used to enumerate Active Directory relationships.

## Usage
- `bloodhound-python --username username --password password --domain domain` : collect all data using default collection methods (includes groups, sessions, and trusts)
- `bloodhound-python --collectionmethod All --kerberos --domain domain` : collect data using Kerberos authentication without requiring a plaintext password
- `bloodhound-python --collectionmethod All --username username --hashes LM:NTLM --domain domain` : authenticate using NTLM hashes instead of a password
- `bloodhound-python --collectionmethod All --username username --password password --domain domain --nameserver nameserver` : specify a custom name server for DNS queries
- `bloodhound-python --collectionmethod All --username username --password password --domain domain --zip` : save the output files as a compressed ZIP archive

## Categories
users, archives, files

## Links
- More information: https://github.com/dirkjanm/BloodHound.py#usage
- tldr page: https://tldr.inbrowser.app/pages/common/bloodhound-python
