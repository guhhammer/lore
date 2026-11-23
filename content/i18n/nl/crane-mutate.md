# crane mutate

Wijzig image-labels en annotaties. De container moet naar een registry worden gepusht, en het manifest wordt daar bijgewerkt.

## Usage
- `crane mutate -a/-l annotation/label` : stel nieuwe annotaties of labels in
- `crane mutate --append/--cmd/--entrypoint/-e/--exposed-ports var1 var2 ...` : voeg een tarball toe, of stel het commando/de entrypoint/omgevingsvariabelen/exposed ports van een image in
- `crane mutate -o pad/naar/tarball` : schrijf de resulterende image naar een nieuwe tarball
- `crane mutate --set-platform platform_naam` : stel het platform van de gewijzigde image in, in de vorm `os/arch/variant:osversion,platform`
- `crane mutate -t tag_naam` : voorzie de gewijzigde image van een nieuwe tagreferentie
- `crane mutate -u gebruikersnaam` : stel een nieuwe gebruiker in
- `crane mutate -w pad/naar/werkmap` : stel een nieuwe werkmap in
- `crane mutate -h` : toon de help
