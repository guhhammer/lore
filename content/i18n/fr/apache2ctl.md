# apache2ctl

L'outil d'Interface en Lignes de Commandes (ILC) pour administrer le serveur web HTTP Apache. Cette commande est disponible sur une distribution Debian. Pour les distributions basées Red Hat, voir `httpd`.

## Usage
- `sudo apache2ctl start` : démarre le démon Apache. Envoie un message s'il est déjà actif
- `sudo apache2ctl stop` : arrête le démon Apache
- `sudo apache2ctl restart` : re-démarre le démon Apache
- `sudo apache2ctl -t` : teste la syntaxe du fichier de configuration
- `sudo apache2ctl -M` : liste les modules chargés

## Links
- More information: https://manned.org/apache2ctl
