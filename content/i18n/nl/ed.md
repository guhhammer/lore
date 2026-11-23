# ed

De originele Unix teksteditor.

## Usage
- `ed` : start een interactieve editor sessie met een leeg document
- `ed -p '> '` : start een interactieve editor sessie met een leeg document en een specifieke prompt
- `ed -v` : start een interactieve editor sessie met gebruiksvriendelijke foutmeldingen
- `ed -q -s` : start een interactieve editor sessie met een leeg document en zonder diagnostics, het aantal bytes en de '!' prompt
- `ed -l` : start een interactieve editor sessie zonder wijziging van de exitstatus als het commando faalt
- `ed pad/naar/bestand` : pas een specifiek bestand aan (dit toont het aantal bytes van het geladen bestand)
- `,s/regex/vervanging/g` : [Interactief] Vervang een string met een specifieke vervanging voor alle regels
- `q` : [Interactief] Sluit `ed` af

## Links
- More information: https://www.gnu.org/software/ed/manual/ed_manual.html
