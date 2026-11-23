# git clone

Kloon een bestaande repository.

## Usage
- `git clone externe_repository_locatie pad/naar/map` : kloon een bestaande repository naar een nieuwe map (de standaardmap is de repository-naam)
- `git clone --recursive externe_repository_locatie` : kloon een bestaande repository en zijn submodules
- `git clone -n externe_repository_locatie` : kloon alleen de `.git`-map van een bestaande repository
- `git clone -l pad/naar/lokale_repository` : kloon een lokale repository
- `git clone -q externe_repository_locatie` : kloon in stille modus
- `git clone --depth 10 externe_repository_locatie` : kloon een bestaande repository, waarbij alleen de nieuwste 10 commits op de standaard-branch worden opgehaald (handig om tijd te besparen)
- `git clone -b naam --single-branch externe_repository_locatie` : kloon een bestaande repository, waarbij alleen een specifieke branch wordt opgehaald
- `git clone -c core.sshCommand="ssh -i pad/naar/privésleutel" externe_repository_locatie` : kloon een bestaande repository met een specifiek SSH-commando

## Links
- More information: https://git-scm.com/docs/git-clone
