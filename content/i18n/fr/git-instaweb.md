# git instaweb

Outil pour le lancement d'un serveur gitweb.

## Usage
- `git instaweb --start` : démarre un serveur gitweb depuis le dépôt (`repository`) courant
- `git instaweb --start -l` : écoute uniquement sur le port localhost
- `git instaweb --start -p 1234` : écoute sur un port spécifique
- `git instaweb --start -d lighttpd|apache2|mongoose|plackup|webrick` : utilise un daemon HTTP spécifique
- `git instaweb --start -b` : lance en même temps qu'un navigateur web
- `git instaweb --stop` : stoppe le serveur
- `git instaweb --restart` : redémarre le serveur

## Links
- More information: https://git-scm.com/docs/git-instaweb
