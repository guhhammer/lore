# dnf config-manager

Beheer DNF-configuratie-opties en repositories op Fedora-gebaseerde systemen. Niet standaard voor `dnf` maar ondersteund via `dnf-plugins-core`.

## Usage
- `dnf config-manager --add-repo=repository_url` : voeg een repository toe (en schakel deze in) vanaf een URL
- `dnf config-manager --dump` : print de huidige configuratiewaarden
- `dnf config-manager --enable repository_id` : schakel een specifieke repository in
- `dnf config-manager --disable repository_id1 repository_id2 ...` : schakel opgegeven repositories uit
- `dnf config-manager --setopt=option=waarde` : stel een configuratieoptie in voor een repository
- `dnf config-manager --help-cmd` : toon de help

## Links
- More information: https://dnf-plugins-core.readthedocs.io/en/latest/config_manager.html
