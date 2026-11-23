# git clone

Klone ein existierendes Repository.

## Usage
- `git clone url_zu_repository pfad/zu/verzeichnis` : klone ein existierendes Repository in ein bestimmtes Verzeichnis
- `git clone --recursive url_zu_repository` : klone ein existierendes Repository und seine Submodule
- `git clone -n url_zu_repository` : klone nur das `.git` Verzeichnis für ein existierendes repository
- `git clone -l pfad/zu/lokalem_repository` : klone ein lokales Repository
- `git clone -q url_zu_repository` : klone ohne Meldungen
- `git clone --depth 10 url_zu_repository` : klone ein existierendes Repository und rufe nur die neuesten 10 Commits im Standard-Branch ab (spart Zeit)
- `git clone -b name --single-branch url_zu_repository` : klone ein existierendes Repository, aber lade nur einen bestimmten Branch herunter
- `git clone -c core.sshCommand="ssh -i pfad/zu/privat_ssh_schlüssel" url_zu_repository` : klone ein existierendes Repository mit einem bestimmten SSH Befehl

## Links
- More information: https://git-scm.com/docs/git-clone
