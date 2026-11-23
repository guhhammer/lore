# git blame

Toon welke commit en auteur elke regel van een bestand voor het laatst heeft gewijzigd.

## Usage
- `git blame pad/naar/bestand` : toon een bestand met informatie over de auteur (auteursnaam en commithash)
- `git blame -e pad/naar/bestand` : toon de e-mail van de auteur in plaats van de naam
- `git blame commit pad/naar/bestand` : toon bestand met auteursnaam en commithash op elke regel op een specifieke commit
- `git blame commit~ pad/naar/bestand` : toon bestand met auteursnaam en commithash op elke regel vóór een specifieke commit
- `git blame -L 123 pad/naar/bestand` : toon een bestand met informatie over de auteur, beginnend bij een gegeven regel
- `git blame -L start_regel,eind_regel pad/naar/bestand` : toon auteursnaam- en commithashinformatie voor een specifiek regelbereik
- `git blame -L '/tekst/',+10 pad/naar/bestand` : annoteer 10 regels van een bestand, beginnend bij de eerste regel die overeenkomt met een gegeven tekenreeks
- `git blame -w -C -C -C pad/naar/bestand` : annoteer een bestand zonder witruimtes en regelverplaatsingen

## Links
- More information: https://git-scm.com/docs/git-blame
