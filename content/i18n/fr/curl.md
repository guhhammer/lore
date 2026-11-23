# curl

Transfère des données depuis ou vers un serveur. Accepte la plupart des protocoles, notamment HTTP, HTTPS, FTP, SCP, etc.

## Usage
- `curl https://example.com` : fait une requête HTTP GET et affiche la réponse
- `curl -L -D - https://example.com` : fait une requête HTTP GET, suit toute redirection HTTP `3xx`, et affiche les en-têtes et le contenu de la réponse vers `stdout`
- `curl -O https://example.com/nom_fichier.zip` : télécharge le contenu d'une URL dans un fichier nommé comme indiqué par l'URL
- `curl -X POST -d 'nom=bob' http://example.com/formulaire` : envoie des données de formulaire encodées (requête POST de type `application/x-www-form-urlencoded`). Utilise `--data @file_name` ou `--data @'-'` pour lire depuis `stdin`
- `curl -k -x http://127.0.0.1:8080 -H 'Authorization: Bearer token' -X GET|PUT|POST|DELETE|PATCH|... https://example.com` : envoie une requête avec un en-tête supplémentaire, en spécifiant la méthode HTTP, à travers un proxy, et en ignorant les erreurs de validation de certificat
- `curl -d '{"nom":"bob"}' -H 'Content-Type: application/json' http://example.com/utilisateurs/1234` : envoie des données au format JSON, en spécifiant l'en-tête Content-Type adéquat
- `curl -E client.pem --key cle.pem -k https://example.com` : transmet le certificat client et la clé privée pour la requête, en ignorant la validation du certificat
- `curl -v --resolve example.com:80:127.0.0.1 http://example.com` : résout un nom de domaine vers une adresse IP spécifique (similaire à modifier le fichier `/etc/hosts` pour une résolution DNS locale), en activant le mode verbeux

## Links
- More information: https://curl.se/docs/manpage.html
