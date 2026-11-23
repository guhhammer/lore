# useradd

Crée un nouvel utilisateur.

## Usage
- `sudo useradd nom_utilisateur` : crée un nouvel utilisateur
- `sudo useradd -u identifiant nom_utilisateur` : crée un nouvel utilisateur en spéficiant un identifiant numérique particulier
- `sudo useradd -s chemin/vers/shell nom_utilisateur` : crée un nouvel utilisateur avec le shell spécifié
- `sudo useradd -G groupe1,groupe2,... nom_utilisateur` : crée un nouvel utilisateur qui appartient aux groupes supplémentaires (attention à l'omission des espaces)
- `sudo useradd -m nom_utilisateur` : crée un nouvel utilisateur avec le répertoire personnel par défaut
- `sudo useradd -k chemin/vers/repertoire_squelette -m nom_utilisateur` : crée un nouvel utilisateur avec un répertoire personnel rempli par les fichiers et répertoires d'un répertoire squelette
- `sudo useradd -r nom_utilisateur` : crée un nouvel utilisateur système sans répertoire personnel

## Links
- More information: https://manned.org/useradd
