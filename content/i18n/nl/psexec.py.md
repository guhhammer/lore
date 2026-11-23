# psexec.py

Voer commando's uit op een Windows-machine op afstand met `RemComSvc`, met PsExec-achtige functionaliteit. Onderdeel van de Impacket-suite.

## Usage
- `psexec.py domein/gebruikersnaam:wachtwoord@doel` : start een interactieve shell op een extern doelwit
- `psexec.py domein/gebruikersnaam:wachtwoord@doel commando` : voer een specifiek commando uit op een doel op afstand
- `psexec.py -c bestandsnaam domein/gebruikersnaam:wachtwoord@doel commando` : kopieer de bestandsnaam voor latere uitvoering, argumenten worden doorgegeven in het commando
- `psexec.py -path "pad\naar\externe_map" domein/gebruikersnaam:wachtwoord@doel commando` : voer een commando uit vanaf een specifiek pad op een extern doelwit
- `psexec.py -hashes LM_Hash:NT_Hash domein/gebruikersnaam@doel` : authenticeer met pass-the-hash-authenticatie in plaats van een wachtwoord
- `psexec.py -k -no-pass domein/gebruikersnaam@doel` : gebruik Kerberos verificatie voor het doel
- `psexec.py -dc-ip domain_controller_ip domein/gebruikersnaam:wachtwoord@doel` : specificeer het IP-adres van de domeincontroller

## Links
- More information: https://github.com/fortra/impacket
