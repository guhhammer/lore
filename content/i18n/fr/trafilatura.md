# trafilatura

Un outil Python pour l'extraction et le crawling de contenu web. Extrait le texte principal, les métadonnées et les commentaires des pages web.

## Usage
- `trafilatura -u url` : extraire le texte d'une URL
- `trafilatura -u url -o chemin/vers/fichier.txt` : extraire le texte et sauve le résultat dans un fichier
- `trafilatura -u url --json` : extraire le texte au format JSON
- `trafilatura -i chemin/vers/liste_urls.txt` : extraire le texte de plusieurs URLs listées dans un fichier
- `trafilatura --sitemap url_vers_sitemap.xml` : crawle un site web en utilisant son sitemap
- `trafilatura -u url --formatting` : extraire le texte en conservant le formatage HTML
- `trafilatura -u url --with-comments` : extraire le texte avec les commentaires
- `trafilatura -h` : affiche l'aide
