# history

Beheer command-line geschiedenis.

## Usage
- `history` : toon de commandogeschiedenis met regelnummers
- `history 20` : toon de laatste 20 commando's (in Zsh worden alle commando's vanaf regel 20 getoond)
- `history -d|f|i|E` : toon geschiedenis met tijdstempels in verschillende formaten (alleen beschikbaar in Zsh)
- `history -c` : wis (c) de commandogeschiedenis
- `history -w` : overschrijf (w) het geschiedenisbestand met de geschiedenis van de huidige Bash-shell (vaak gecombineerd met `history -c` om geschiedenis te wissen)
- `history -d offset` : verwijder een commando uit de geschiedenis op de opgegeven offset
- `history -s commando` : voeg een commando toe aan de geschiedenis zonder het uit te voeren
- `<Spatie>commando` : voer een commando uit zonder het toe te voegen aan de geschiedenis door te beginnen met een spatie

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-history
