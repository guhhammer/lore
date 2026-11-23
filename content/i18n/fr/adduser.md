# adduser

Outil d'ajout d'utilisateurs.

## Usage
- `adduser nom_utilisateur` : crée un nouvel utilisateur avec un répertoire personnel générique et demande interactivement un mot de passe
- `adduser --no-create-home nom_utilisateur` : crée un nouvel utilisateur sans répertoire personnel
- `adduser --home chemin/vers/répertoire nom_utilisateur` : crée un nouvel utilisateur avec un répertoire personnel correspondant au répertoire spécifié
- `adduser --shell chemin/vers/shell nom_utilisateur` : crée un nouvel utilisateur avec l'interpréteur de commandes spécifié comme interpréteur de commandes de connexion
- `adduser --ingroup groupe nom_utilisateur` : crée un nouvel utilisateur appartenant au groupe donné

## Links
- More information: https://manned.org/adduser
