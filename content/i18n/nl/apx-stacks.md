# apx stacks

Beheer stapels in `apx`. Opmerking: door gebruikers gecreëerde stapelconfiguraties worden opgeslagen in `~/.local/share/apx/stacks`.

## Usage
- `apx stacks new` : maak interactief een nieuwe stapelconfiguratie
- `apx stacks update naam` : update interactief een stapelconfiguratie
- `apx stacks list` : toon alle beschikbare stapelconfiguraties
- `apx stacks rm --name string` : verwijder een specifieke stapelconfiguratie
- `apx stacks import --input pad/naar/stack.yml` : importeer een stapelconfiguratie
- `apx stacks export --name string --output pad/naar/output_bestand` : exporteer de stapelconfiguratie (Let op: de output flag is optioneel, het wordt standaard geëxporteerd naar de huidige map)

## Links
- More information: https://docs.vanillaos.org/docs/en/apx-manpage#stacks
