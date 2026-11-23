# wget

Télécharge des fichiers depuis le Web. Prend en charge HTTP, HTTPS et FTP.

## Usage
- `wget https://example.com/foo` : télécharge le contenu d'une URL vers un fichier (nommé « foo » dans ce cas)
- `wget -O chemin/vers/fichier https://example.com/foo` : télécharge le contenu d'une URL vers un fichier spécifique
- `wget -pkw 3 https://example.com/une_page.html` : télécharge une page web et toutes ses ressources avec des intervalles de 3 secondes entre les requêtes (scripts, feuilles de style, images, etc.)
- `wget -mnp https://example.com/un_chemin/` : télécharge tous les fichiers listés dans un répertoire et ses sous-répertoires (ne télécharge pas les éléments intégrés de la page)
- `wget --limit-rate 300k -t 100 https://example.com/un_chemin/` : limite la vitesse de téléchargement et le nombre de tentatives de connexion
- `wget --user nom_utilisateur --password mot_de_passe https://example.com` : télécharge un fichier depuis un serveur HTTP avec l'authentification Basic (fonctionne aussi pour FTP)
- `wget -c https://example.com` : reprend un téléchargement incomplet
- `wget -P chemin/vers/répertoire -i chemin/vers/URLs.txt` : télécharge toutes les URLs stockées dans un fichier texte vers un répertoire spécifique

## Links
- More information: https://www.gnu.org/software/wget/manual/wget.html
