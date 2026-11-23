# archinstall

Begeleide Arch Linux-installer.

## Usage
- `archinstall` : start de interactieve installatie
- `archinstall --dry-run` : simuleer een installatie door de interactieve installer te doorlopen, en genereer een configuratiebestand zonder te installeren
- `archinstall --advanced` : schakel geavanceerde instellingen in
- `archinstall --config pad/naar/config.json --creds pad/naar/credentials.json` : installeer met de opgegeven configuratiebestanden
- `archinstall --config-url https://example.com/config.json --creds-url https://example.com/credentials.json` : installeer met configuratiebestanden van een externe server
- `archinstall --script minimal|only_hd` : installeer met behulp van het opgegeven script

## Links
- More information: https://archinstall.archlinux.page/installing/guided.html
