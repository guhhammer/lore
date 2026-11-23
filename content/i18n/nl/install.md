# install

Kopieer bestanden en stel attributen in. Meestal gebruikt door Makefiles.

## Usage
- `install pad/naar/bronbestand1 pad/naar/bronbestand2 ... pad/naar/bestemming` : kopieer bestanden naar de bestemming
- `install -o gebruiker pad/naar/bronbestand1 pad/naar/bronbestand2 ... pad/naar/bestemming` : kopieer bestanden naar de bestemming en stel hun eigendom in
- `install -g gebruiker pad/naar/bronbestand1 pad/naar/bronbestand2 ... pad/naar/bestemming` : kopieer bestanden naar de bestemming en stel hun groepseigendom in
- `install -m +x pad/naar/bronbestand1 pad/naar/bronbestand2 ... pad/naar/bestemming` : kopieer bestanden naar de bestemming en stel hun `modus` in
- `install -p pad/naar/bronbestand1 pad/naar/bronbestand2 ... pad/naar/bestemming` : kopieer bestanden en pas toegangstijden/wijzigingstijden van de bron toe op de bestemming
- `install -D pad/naar/bronbestand1 pad/naar/bronbestand2 ... pad/naar/bestemming` : kopieer bestanden en maak de mappen op de bestemming aan als ze niet bestaan
