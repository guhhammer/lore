# tput

Bekijk en wijzig terminalinstellingen en -mogelijkheden.

## Usage
- `tput cup rij kolom` : verplaats de cursor naar een schermlocatie
- `tput setaf|setab ansi_kleurcode` : stel de voorgrond- (af) of achtergrondkleur (ab) in
- `tput rev` : verwissel tekst- en achtergrondkleuren
- `tput sgr0` : reset alle tekstattributen van de terminal
- `tput cols|lines|colors` : toon het aantal kolommen, regels of kleuren
- `tput smam|rmam` : schakel woordomloop in of uit
- `tput civis|cnorm` : verberg of toon de terminalcursor
- `tput smcup|rmcup` : sla de tekststatus van de terminal op of herstel deze (smcup legt ook scrollwielgebeurtenissen vast)

## Links
- More information: https://manned.org/tput
