# usermod

Modifie le compte d'un utilisateur.

## Usage
- `sudo usermod -l nouveau_nom nom_utilisateur` : change le nom d'un utilisateur
- `sudo usermod -u identifiant nom_utilisateur` : modifie l'identifiant numérique d'un utilisateur
- `sudo usermod -s chemin/vers/shell nom_utilisateur` : change le shell d'un utilisateur
- `sudo usermod -a -G groupe1,groupe2,... nom_utilisateur` : ajoute l'utilisateur à des groupes supplémentaires (attention à l'omission d'espaces)
- `sudo usermod -m -d chemin/vers/nouveau_répertoire nom_utilisateur` : change le répertoire personnel d'un utilisateur et déplace ses fichiers vers celui-ci

## Links
- More information: https://manned.org/usermod
