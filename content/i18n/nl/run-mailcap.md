# run-mailcap

Voer programma's uit via vermeldingen in het mailcap-bestand.

## Usage
- `run-mailcap --action=compose pad/naar/bestand` : bewerk een bestaand of nieuw bestand met de standaard `mailcap`-bewerktool
- `run-mailcap --action=edit pad/naar/bestand` : bekijk een bestand met de standaard `mailcap`-verkenner
- `run-mailcap --action=print pad/naar/bestand` : print een bestand met de standaard `mailcap`-printtool
- `run-mailcap --action=view pad/naar/bestand` : bekijk een bestand (meestal een afbeelding) met de standaard `mailcap`-verkenner
- `run-mailcap --action=view|cat|compose|composetyped|edit|print pad/naar/bestand` : roep afzonderlijke acties/programma's aan via run-mailcap
- `run-mailcap --action=actie --debug pad/naar/bestand` : schakel extra informatie in
- `run-mailcap --action=actie --nopager pad/naar/bestand` : negeer elke "copiousoutput"-richtlijn en stuur de uitvoer door naar `stdout`
- `run-mailcap --action=actie --norun pad/naar/bestand` : toon het gevonden commando zonder het daadwerkelijk uit te voeren

## Links
- More information: https://manned.org/run-mailcap
