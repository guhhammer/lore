# atuin

Enregistre votre historique shell dans une base de donnée. Peut également synchroniser votre historique chiffré entre plusieurs machines.

## Usage
- `eval "$(atuin init bash|zsh|fish)"` : installe atuin dans votre shell
- `atuin import auto` : importe l'historique du shell par défaut
- `atuin search commande` : recherche dans l'historique shell une commande spécifique
- `atuin register -u nom_utilisateur -e email -p mot_de_passe` : inscrit un compte sur le serveur de synchronisation par défaut
- `atuin login -u nom_utilisateur -p mot_de_passe` : se connecte au serveur de synchronisation par défaut
- `atuin sync` : synchronise l'historique avec le serveur

## Links
- More information: https://docs.atuin.sh/
