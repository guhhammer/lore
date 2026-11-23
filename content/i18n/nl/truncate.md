# truncate

Verkort of verleng de grootte van een bestand naar de opgegeven grootte.

## Usage
- `truncate -s 10G pad/naar/bestand` : stel een grootte van 10 GB in voor een bestaand bestand, of maak een nieuw bestand met de opgegeven grootte
- `truncate -s +50M pad/naar/bestand` : verleng de bestandsgrootte met 50 MiB, vul met gaten (die lezen als nulbytes)
- `truncate -s -2G pad/naar/bestand` : verkort het bestand met 2 GiB door gegevens van het einde van het bestand te verwijderen
- `truncate -s 0 pad/naar/bestand` : leeg de inhoud van het bestand
- `truncate -s 0 -c pad/naar/bestand` : leeg de inhoud van het bestand, maar maak het bestand niet aan als het niet bestaat
