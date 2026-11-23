# ab

Outil d'analyse pour serveur Apache HTTP.

## Usage
- `ab -n 100 url` : exécute 100 requêtes HTTP GET sur une URL donnée
- `ab -n 100 -c 10 url` : exécute 100 requêtes HTTP GET en parallèle par groupe de 10, sur une URL
- `ab -n 100 -T application/json -p chemin/vers/le/fichier.json url` : exécute 100 requêtes HTTP POST sur une URL, en utilisant un contenu JSON depuis un fichier
- `ab -k url` : utilise la fonctionalitée HTTP Keep Alive pour exécuter plusieurs requêtes dans la même session HTTP
- `ab -t 60 url` : fixe le nombre maximum de secondes d'exécution pour l'analyse

## Links
- More information: https://httpd.apache.org/docs/current/programs/ab.html
