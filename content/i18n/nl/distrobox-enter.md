# distrobox-enter

Betreed een Distrobox-container. Standaard commando dat wordt uitgevoerd is je `$SHELL`, maar je kan verschillende shells of hele commando's specificeren. Indien gebruikt in een script/applicatie/service, kan je de `--headless`-modus gebruiken om de tty en interactiviteit uit te schakelen.

## Usage
- `distrobox-enter container_naam` : betreed een Distrobox-container
- `distrobox-enter container_naam -- sh -l` : betreed een Distrobox-container en voer een commando uit bij het inloggen
- `distrobox-enter -n container_naam -- uptime --pretty` : betreed een Distrobox-container zonder een tty te instantiëren

## Links
- More information: https://distrobox.it/usage/distrobox-enter/
