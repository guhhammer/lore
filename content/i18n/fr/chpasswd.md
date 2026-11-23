# chpasswd

Modifie les mots de passe de plusieurs utilisateurs en utilisant `stdin`.

## Usage
- `printf "nom_utilisateur:nouveau_mot_de_passe" | sudo chpasswd` : modifie le mot de passe d'un utilisateur spécifique
- `printf "nom_utilisateur_1:nouveau_mot_de_passe_1\nnom_utilisateur_2:nouveau_mot_de_passe_2" | sudo chpasswd` : modifie les mots de passe de plusieurs utilisateurs (le texte d'entrée ne doit pas contenir d'espaces)
- `printf "nom_utilisateur:nouveau_mot_de_passe_chiffré" | sudo chpasswd -e` : modifie le mot de passe d'un utilisateur spécifique, et le spécifie sous forme chiffrée
- `printf "nom_utilisateur:nouveau_mot_de_passe" | sudo chpasswd -c NONE|DES|MD5|SHA256|SHA512` : modifie le mot de passe d'un utilisateur spécifique, et utilise un algorithme de chiffrement spécifique pour le mot de passe stocké

## Links
- More information: https://manned.org/chpasswd
