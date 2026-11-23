# smbserver.py

Host SMB-shares. Onderdeel van de Impacket-suite.

## Usage
- `smbserver.py share_naam pad/naar/share` : stel een basis SMB-share in
- `smbserver.py -comment mijn_share share_naam pad/naar/share` : stel een share in met een aangepast commentaar
- `smbserver.py -username gebruikersnaam -password wachtwoord share_naam pad/naar/share` : stel een share in met gebruikersnaam en wachtwoord verificatie
- `smbserver.py -hashes LMHASH:NTHASH share_naam pad/naar/share` : stel een share in met NTLM hash-authenticatie
- `smbserver.py -ip interface_ip_adres share_naam pad/naar/share` : stel een share in op een specifieke interface
- `smbserver.py -port poort share_naam pad/naar/share` : stel een share in op een niet-standaard SMB-poort
- `smbserver.py -smb2support share_naam pad/naar/share` : stel een share in met SMB2 ondersteuning
- `smbserver.py -outputfile pad/naar/uitvoerbestand share_naam pad/naar/share` : stel een share in en log de commando's in een uitvoerbestand

## Links
- More information: https://github.com/fortra/impacket
