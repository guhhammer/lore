# stty

Stel opties in of verkrijg opties voor een terminalapparaatinterface.

## Usage
- `stty size` : toon de huidige terminalgrootte
- `stty -a` : toon alle instellingen voor de huidige terminal
- `stty rows|cols aantal` : stel het aantal rijen of kolommen in
- `stty -F pad/naar/apparaat_bestand speed` : verkrijg de daadwerkelijke overdrachtssnelheid van een apparaat
- `stty sane` : reset alle modi naar redelijke waarden voor de huidige terminal
- `stty raw|cooked` : wissel tussen rauwe en normale modus
- `stty -echo|echo` : zet karakter echoing uit of aan
- `stty --help` : toon de help

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/stty-invocation.html
