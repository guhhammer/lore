# fish

De Friendly Interactive SHell, een commandoregel-interpreteerder die is ontworpen voor gebruiksvriendelijkheid.

## Usage
- `fish` : start een interactieve shellsessie
- `fish -N` : start een interactieve shellsessie zonder opstartconfiguraties te laden
- `fish -c "echo 'fish is executed'"` : voer specifieke commando's uit
- `fish pad/naar/script.fish` : voer een specifiek script uit
- `fish -n pad/naar/script.fish` : controleer een specifiek script op syntax fouten
- `echo "echo 'fish is executed'" | fish` : voer specifieke commando's uit van `stdin`
- `fish -P` : start een interactieve shellsessie in privémodus, waarbij de shell geen toegang heeft tot oude geschiedenis of nieuwe geschiedenis opslaat
- `set -U -x variabele_naam variabele_waarde` : definieer en exporteer een omgevingsvariabele die blijft na het herstarten van de shell (ingebouwd)

## Links
- More information: https://fishshell.com/docs/current/cmds/fish.html
