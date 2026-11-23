# usermod

Wijzig een gebruikersaccount.

## Usage
- `sudo usermod -l nieuwe_gebruikersnaam gebruikersnaam` : verander een gebruikersnaam
- `sudo usermod -u gebruiker_id gebruikersnaam` : verander een gebruikers-ID
- `sudo usermod -s pad/naar/shell gebruikersnaam` : verander een gebruikersshell
- `sudo usermod -aG groep1,groep2,... gebruikersnaam` : voeg een gebruiker toe aan aanvullende groepen (let op het ontbreken van spaties)
- `sudo usermod -rG groep1,groep2,... gebruikersnaam` : verwijder een gebruiker uit specifieke groepen
- `sudo usermod -m -d pad/naar/nieuwe_thuismap gebruikersnaam` : verander een gebruikersthuismap
- `sudo usermod -L gebruikersnaam` : vergrendel een account
- `sudo usermod -U gebruikersnaam` : ontgrendel een account

## Links
- More information: https://manned.org/usermod
