# paste

Voeg regels van bestanden samen.

## Usage
- `paste -s pad/naar/bestand` : voeg alle regels samen tot één enkele regel, met `TAB` als scheidingsteken
- `paste -s -d scheidingsteken pad/naar/bestand` : voeg alle regels samen tot één enkele regel, met het opgegeven scheidingsteken
- `paste pad/naar/bestand1 pad/naar/bestand2` : voeg twee bestanden zij aan zij samen, elk in zijn kolom, met `TAB` als scheidingsteken
- `paste -d scheidingsteken pad/naar/bestand1 pad/naar/bestand2` : voeg twee bestanden zij aan zij samen, elk in zijn kolom, met het opgegeven scheidingsteken
- `paste -d '\n' pad/naar/bestand1 pad/naar/bestand2` : voeg twee bestanden samen, met afwisselend toegevoegde regels
