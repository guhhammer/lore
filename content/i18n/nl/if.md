# if

Voert voorwaardelijke verwerking uit in shell-scripts.

## Usage
- `if voorwaarde_commando; then echo "Voorwaarde is waar"; fi` : voer de opgegeven commando's uit als de exitstatus van het voorwaardelijke commando nul is
- `if ! voorwaarde_commando; then echo "Voorwaarde is waar"; fi` : voer de opgegeven commando's uit als de exitstatus van het voorwaardelijke commando niet nul is
- `if voorwaarde_commando; then echo "Voorwaarde is waar"; else echo "Voorwaarde is onwaar"; fi` : voer de eerste opgegeven commando's uit als de exitstatus van het voorwaardelijke commando nul is, anders voer de tweede opgegeven commando's uit
- `if [[ -f pad/naar/bestand ]]; then echo "Voorwaarde is waar"; fi` : controleer of een bestand (f) bestaat
- `if [[ -d pad/naar/map ]]; then echo "Voorwaarde is waar"; fi` : controleer of een map (d) bestaat
- `if [[ -e pad/naar/bestand_of_map ]]; then echo "Voorwaarde is waar"; fi` : controleer of een bestand of map bestaat
- `if [[ -n "$variabele" ]]; then echo "Voorwaarde is waar"; fi` : controleer of een variabele is gedefinieerd
- `man test` : toon alle mogelijke voorwaarden (`test` is een alias voor `[`; beide worden vaak gebruikt met `if`)

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#Conditional-Constructs
