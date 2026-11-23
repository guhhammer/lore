# clear-history

Verwijder vermeldingen uit de PowerShell-sessie commandogeschiedenis.

## Usage
- `Clear-History` : verwijder alle geschiedenis van de huidige sessie
- `Clear-History -CommandLine "commando"` : verwijder een commando op specifieke naam
- `Clear-History -CommandLine "commando1", "commando2", ...` : verwijder meerdere commando's op naam
- `Clear-History -Id id_nummer` : verwijder een specifiek geschiedenisvermelding op ID
- `Clear-History -Id id1, id2, ...` : verwijder meerdere ID's
- `Clear-History -Id (start_id..eind_id)` : verwijder commando's binnen een bereik van ID's
- `Clear-History -WhatIf` : simuleer het verwijderen van de geschiedenis en toon wat er verwijderd zou worden
- `Clear-History -Confirm` : vraag om bevestiging voor het verwijderen
