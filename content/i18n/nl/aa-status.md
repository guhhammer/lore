# aa-status

Toon de momenteel geladen AppArmor-modules.

## Usage
- `sudo aa-status` : controleer de status
- `sudo aa-status --json` : toon status in JSON-formaat
- `sudo aa-status --pretty-json` : toon status in geformatteerd JSON-formaat
- `sudo aa-status --profiled` : toon het aantal geladen profielen
- `sudo aa-status --enforced` : toon het aantal geladen afdwingende profielen
- `sudo aa-status --complaining` : toon het aantal geladen niet-afdwingende profielen
- `sudo aa-status --kill` : toon het aantal geladen afdwingende profielen die taken beëindigen

## Links
- More information: https://gitlab.com/apparmor/apparmor/-/wikis/manpage_aa-status.8
