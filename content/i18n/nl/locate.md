# locate

Vind snel bestandsnamen.

## Usage
- `locate "patroon"` : zoek naar een patroon in de database. Opmerking: de database wordt periodiek herberekend (meestal wekelijks of dagelijks)
- `locate -i "patroon"` : zoek naar een patroon zonder op hoofdletters te letten
- `locate "*/bestandsnaam"` : zoek naar een bestand op basis van de exacte bestandsnaam (een patroon zonder globtekens wordt geïnterpreteerd als `*patroon*`)
- `sudo updatedb` : herbereken de database. Dit moet je doen als je recent toegevoegde bestanden wilt vinden

## Links
- More information: https://manned.org/locate
