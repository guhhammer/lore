# smbclient.py

Interacteer met SMB-servers. Onderdeel van de Impacket-suite.

## Usage
- `smbclient.py domein/gebruikersnaam:wachtwoord@doel` : maak verbinding met een SMB-server met gebruikersnaam en wachtwoord
- `smbclient.py -hashes LM_HASH:NT_HASH domein/gebruikersnaam@doel` : maak verbinding met NTLM hashes voor authenticatie
- `smbclient.py -k domein/gebruikersnaam@doel` : maak verbinding met Kerberos-authenticatie
- `smbclient.py -dc-ip domain_controller_ip domein/gebruikersnaam:wachtwoord@doel` : maak verbinding door een domeincontroller-IP op te geven
- `smbclient.py -target-ip doel_ip domein/gebruikersnaam:wachtwoord@doel` : maak verbinding met een specifiek doel-IP in plaats van NetBIOS-naam
- `smbclient.py -port poort domein/gebruikersnaam:wachtwoord@doel` : maak verbinding met een niet-standaard SMB-poort
- `smbclient.py -inputfile pad/naar/invoerbestand domein/gebruikersnaam:wachtwoord@doel` : voer opdrachten uit vanuit een invoerbestand in de SMB-shell
- `smbclient.py -outputfile pad/naar/uitvoerbestand domein/gebruikersnaam:wachtwoord@doel` : log SMB-clientopdrachten naar een uitvoerbestand

## Links
- More information: https://github.com/fortra/impacket
