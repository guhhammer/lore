# amass enum

Trouve les sous-domaines d'un domaine.

## Usage
- `amass enum -passive -d nom_de_domaine` : trouve les sous-domaines d'un domaine passivement
- `amass enum -active -d nom_de_domaine -p 80,443,8080` : trouve les sous-domaines d'un domaine et les verifie activement en essayant de résoudre les sous-domaines trouvés
- `amass enum -brute -d nom_de_domaine` : fait recherche par force brute pour les sous-domaines
- `amass enum -o fichier_de_sortie -d nom_de_domaine` : sauvegarde les résultats vers un fichier texte
- `amass enum -o fichier_de_sortie -dir chemin/vers/la_base_de_données` : sauvegarde les résultats dans une base de données
