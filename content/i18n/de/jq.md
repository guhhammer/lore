# jq

Ein JSON-Verarbeiter für die Kommandozeile mit einer domänenspezifischen Sprache.

## Usage
- `cat pfad/zu/datei.json | jq '.'` : führe den angegebenen Ausdruck aus (gib farbiges und formatiertes JSON aus)
- `cat pfad/zu/datei.json | jq --from-file pfad/zu/skript.jq` : führe ein gegebenes Skript aus
- `cat pfad/zu/datei.json | jq --arg "name1" "wert1" --arg "name2" "wert2" ... '. + $ARGS.named'` : übergib bestimmte Argumente
- `cat pfad/zu/datei.json | jq '.schlüssel1, .schlüssel2, ...'` : gib bestimmte Schlüssel aus
- `cat pfad/zu/datei.json | jq '.[index1], .[index2], ...'` : gib bestimmte Listenelemente aus
- `cat pfad/zu/datei.json | jq '.[]'` : gib alle Listenelemente/Objektschlüssel aus
- `cat pfad/zu/datei.json | jq '. +|- {"schlüssel1": "wert1", "schlüssel2": "wert2", ...}'` : füge bestimmte Schlüssel hinzu/lösche bestimmte Schlüssel

## Links
- More information: https://jqlang.org/manual/
