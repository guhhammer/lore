# git commit

Commit bestanden naar de repository.

## Usage
- `git commit` : open een tekstbewerker om een bericht te schrijven en commit de toegevoegde bestanden naar de repository
- `git commit -m "bericht"` : commit toegevoegde bestanden naar de repository met het opgegeven bericht
- `git commit -F pad/naar/commit_bericht_bestand` : commit toegevoegde bestanden met een bericht dat uit een bestand wordt gelezen
- `git commit -a -m "bericht"` : voeg automatisch alle bewerkte en verwijderde bestanden toe en commit
- `git commit -S sleutel_id -m "bericht"` : commit toegevoegde bestanden en onderteken ze met de opgegeven GPG-sleutel (of de sleutel die is gedefinieerd in het configuratiebestand als er geen `sleutel_id` is opgegeven)
- `git commit --amend` : voeg de huidige wijzigingen toe aan de laatste commit en herschrijf deze, waarbij de commit hash wordt aangepast en een tekstbewerker wordt geopend om het bericht te veranderen
- `git commit pad/naar/bestand1 pad/naar/bestand2 ...` : commit alleen specifieke (al toegevoegde) bestanden
- `git commit -m "bericht" -m "omschrijving"` : maak een commit met het opgegeven bericht en omschrijving

## Links
- More information: https://git-scm.com/docs/git-commit
