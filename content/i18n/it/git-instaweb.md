# git instaweb

Helper per avviare un server gitweb.

## Usage
- `git instaweb --start` : avvia un server gitweb dal repository corrente
- `git instaweb --start -l` : resta in ascolto solo su localhost
- `git instaweb --start -p 1234` : resta in ascolto su una porta specifica
- `git instaweb --start -d lighttpd|apache2|mongoose|plackup|webrick` : usa un HTTP daemon specifico
- `git instaweb --start -b` : avvia automaticamente anche un web browser
- `git instaweb --stop` : interrompi il server gitweb in esecuzione
- `git instaweb --restart` : riavvia il server gitweb in esecuzione

## Links
- More information: https://git-scm.com/docs/git-instaweb
