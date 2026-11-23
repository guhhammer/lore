# vcsh

Versiebeheersysteem voor de home-directory met behulp van Git-repositories.

## Usage
- `vcsh init repository_naam` : initialiseer een (lege) repository
- `vcsh clone git_url repository_naam` : kloon een repository naar een aangepaste mapnaam
- `vcsh list` : toon alle beheerde repositories
- `vcsh repository_naam git_commando` : voer een Git-commando uit op een beheerde repository
- `vcsh push|pull` : push/pull alle beheerde repositories naar/van remotes
- `vcsh write-gitignore repository_naam` : schrijf een aangepast `.gitignore`-bestand voor een beheerde repository

## Links
- More information: https://manned.org/vcsh
