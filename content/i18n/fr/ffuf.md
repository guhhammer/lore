# ffuf

Un outil d'énumération web rapide écrit en Go. Le mot-clé `FUZZ` est utilisé comme substitut. `ffuf` essaiera d'atteindre l'URL en remplaçant le mot `FUZZ` par tous les mots de la liste donnée.

## Usage
- `ffuf -c -w chemin/vers/liste.txt -u http://cible/FUZZ` : énumère les répertoires en utilisant une sortie colorée et une liste (wordlist) spécifiant une uRL cible
- `ffuf -w chemin/vers/sous-domaines.txt -u http://FUZZ.cible.com` : énumère les sous-domaines en changeant la position du mot-clé
- `ffuf -o -w chemin/vers/liste.txt -u http://cible/FUZZ -t 500 -x http://127.0.0.1:8080` : énumère avec le nombre de fils d'exécution (threads) spécifié (par défaut : 40) et passe le trafic par un serveur mandataire (proxy) et sauvegarde la sortie (output) dans un fichier
- `ffuf -w chemin/vers/liste.txt -u http://cible.com -H "Host: FUZZ" -mc 200` : énumère dans un en-tête (Header) spécifique ("Nom : Valeur") et n'affiche que les codes d'état HTTP correspondant (match)
- `ffuf -w chemin/vers/postdata.txt -X POST -d "username=admin\&password=FUZZ" -u http://cible/login.php -fc 401,403` : énumère en spécifiant une méthode HTTP spécifiée et des données, tout en filtrant les codes d'état séparés par des virgules

## Links
- More information: https://github.com/ffuf/ffuf#usage
