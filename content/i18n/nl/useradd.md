# useradd

Maak een nieuwe gebruiker aan.

## Usage
- `sudo useradd gebruikersnaam` : maak een nieuwe gebruiker aan
- `sudo useradd -u gebruiker_id gebruikersnaam` : maak een nieuwe gebruiker aan met de opgegeven gebruikers-ID
- `sudo useradd -s pad/naar/shell gebruikersnaam` : maak een nieuwe gebruiker aan met de opgegeven shell
- `sudo useradd -G groep1,groep2,... gebruikersnaam` : maak een nieuwe gebruiker aan die behoort tot extra groepen (let op het ontbreken van spaties)
- `sudo useradd -m gebruikersnaam` : maak een nieuwe gebruiker aan met de standaard thuismap
- `sudo useradd -k pad/naar/sjabloonmap -m gebruikersnaam` : maak een nieuwe gebruiker aan met de thuismap gevuld met bestanden uit een sjabloonmap
- `sudo useradd -r gebruikersnaam` : maak een nieuwe systeemgebruiker aan zonder thuismap

## Links
- More information: https://manned.org/useradd
