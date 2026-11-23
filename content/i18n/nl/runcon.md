# runcon

Voer een programma uit in een andere SELinux-beveiligingscontext.

## Usage
- `runcon` : toon de beveiligingscontext van de huidige uitvoeringscontext
- `runcon -t domein_t commando` : specificeer het domein om een commando in uit te voeren
- `runcon -r rol_r commando` : specificeer de context rol om een commando mee uit te voeren
- `runcon gebruiker_u:rol_r:domein_t commando` : specificeer de volledige context om een commando mee uit te voeren
