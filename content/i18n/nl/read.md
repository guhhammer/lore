# read

Shell builtin voor het ophalen van data van `stdin`.

## Usage
- `read variabele1 variabele2 ...` : sla gegevens op die je van het toetsenbord typt in een of meerdere variabelen
- `read -a array` : sla elk van de volgende regels die je invoert op als waarden van een array
- `read -n aantal_karakters variabele` : specificeer het maximale aantal karakters dat gelezen moet worden
- `read <<<"De achternaam is Bond" _ variabele1 _ variabele2` : wijs meerdere waarden toe aan meerdere variabelen
- `read -r variabele` : laat backslash (`\`) niet optreden als een escape-teken
- `read -p "Voer je invoer hier in: " variabele` : toon een prompt vóór de invoer
- `read -s variabele` : echo de ingetikte tekens niet (stille modus)
- `commando | while IFS= read -r line; do echo|ls|rm|... "$line"; done` : voer een actie uit op elke regel van de uitvoer van een commando

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-read
