# shred

Overschrijf bestanden om gegevens veilig te verwijderen.

## Usage
- `shred pad/naar/bestand` : overschrijf een bestand
- `shred -v pad/naar/bestand` : overschrijf een bestand en toon de voortgang op het scherm
- `shred -z pad/naar/bestand` : overschrijf een bestand, waarbij nullen in plaats van willekeurige gegevens worden achtergelaten
- `shred -n 25 pad/naar/bestand` : overschrijf een bestand een specifiek aantal keren
- `shred -u pad/naar/bestand` : overschrijf een bestand en verwijder het
- `shred -vzu -n 100 pad/naar/bestand` : overschrijf een bestand 100 keer, voeg een laatste overschrijving met nullen toe, verwijder het bestand na overschrijven en toon verbose voortgang op het scherm
