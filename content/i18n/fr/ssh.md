# ssh

Secure Shell est un protocole utilisé pour se connecter de façon sécurisée à des systèmes distants. On peut l'utiliser pour se connecter ou exécuter des commandes sur un serveur distant.

## Usage
- `ssh utilisateur@hôte_distant` : se connecte à un serveur distant
- `ssh utilisateur@hôte_distant -i chemin/vers/fichier_clé` : se connecte à un serveur distant en utilisant une identité spécifique (clé privée)
- `ssh utilisateur@10.0.0.1 -p 2222` : se connecte à un serveur distant avec IP `10.0.0.1` et utilisant un port spécifique (Remarque : `10.0.0.1` peut être réduit à `10.1`)
- `ssh utilisateur@hôte_distant -t commande arguments_commande` : exécute une commande sur un serveur distant avec une allocation tty qui permet une interaction avec la commande distante
- `ssh utilisateur@hôte_distant -D 1080` : tunnel SSH : Redirection de port Dynamique (le proxy SOCKS se trouve sur `localhost:1080`)
- `ssh utilisateur@hôte_distant -L 9999:example.org:80 -N -T` : tunnel SSH : Transfère un port spécifique (`localhost:9999` vers `example.org:80`) en désactivant l'allocation de pseudo-Tty et l'exécutioN de commandes distantes
- `ssh utilisateur@hôte_distant -J utilisateur@hôte_de_rebond` : saut SSH : Se connecte à un serveur distant à travers une machine de rebond (plusieurs machines de rebond peuvent être définies en les séparant par des virgules)
- `<Enter><~><.>` : ferme une session bloquée

## Links
- More information: https://man.openbsd.org/ssh
