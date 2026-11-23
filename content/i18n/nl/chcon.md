# chcon

Verander SELinux-beveiligingscontext van een bestand of bestanden/mappen.

## Usage
- `ls -lZ pad/naar/bestand` : toon beveiligingscontext van een bestand
- `chcon --reference referentiebestand doelbestand` : verander de beveiligingscontext van een doelbestand, door gebruik te maken van een referentiebestand
- `chcon gebruiker:rol:type:bereik/niveau bestandsnaam` : verander de volledige SELinux-beveiligingscontext van een bestand
- `chcon -u gebruiker bestandsnaam` : verander alleen het gebruikersgedeelte van de SELinux-beveiligingscontext
- `chcon -r rol bestandsnaam` : verander alleen het rolgedeelte van de SELinux-beveiligingscontext
- `chcon -t type bestandsnaam` : verander alleen het typegedeelte van de SELinux-beveiligingscontext
- `chcon -l bereik/niveau bestandsnaam` : verander alleen het bereik/niveaugedeelte van de SELinux-beveiligingscontext
