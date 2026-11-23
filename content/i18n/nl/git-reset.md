# git reset

Maak commits of niet-toegevoegde wijzigingen ongedaan door de huidige Git HEAD te herstellen naar de opgegeven status. Als een pad is opgegeven, werkt dit als "unstage"; als een commit-hash of branch is meegegeven, werkt dit als "uncommit".

## Usage
- `git reset` : maak alle toevoegingen ongedaan
- `git reset pad/naar/bestand1 pad/naar/bestand2 ...` : maak toevoegingen van bepaalde bestand(en) ongedaan
- `git reset -p pad/naar/bestand` : maak toevoegingen van delen van een bestand interactief ongedaan
- `git reset HEAD~` : maak de laatste commit ongedaan, waarbij de wijzigingen (en alle andere ongecommitte wijzigingen) in het bestandssysteem blijven
- `git reset --soft HEAD~2` : maak de laatste twee commits ongedaan en voeg hun veranderingen toe aan de index, d.w.z. toegevoegd voor commit
- `git reset --hard` : verwijder alle ongecommitte veranderingen, toegevoegd of niet (voor alleen niet-toegevoegde wijzigingen, gebruik `git checkout`)
- `git reset --hard commit` : herstel de repository naar een bepaalde commit, waarbij gecommitte, toegevoegde en niet-gecommitte wijzigingen sindsdien worden verwijderd

## Links
- More information: https://git-scm.com/docs/git-reset
