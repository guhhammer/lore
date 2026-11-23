# ansible-vault

Chiffre & déchiffre des valeurs, des structures de données et des fichiers dans un projet Ansible.

## Usage
- `ansible-vault create fichier_vault` : crée un nouveau fichier vault chiffré avec une invite à rentrer un mot passe
- `ansible-vault create --vault-password-file fichier_de_mot_de_passe fichier_vault` : crée un nouveau fichier vault chiffré avec un fichier clé vault pour le chiffrer
- `ansible-vault encrypt --vault-password-file fichier_de_mot_de_passe fichier_vault` : chiffre un ficher existant avec un fichier de mot de passe optionnel
- `ansible-vault encrypt_string` : chiffre un texte avec le format de chiffrage pour textes d'Ansible, en affichant une invite interactif
- `ansible-vault view --vault-password-file fichier_de_mot_de_passe fichier_vault` : affiche un fichier chiffré, en utilisant un fichier de mot de passe pour le déchiffrer
- `ansible-vault rekey --vault-password-file ancien_fichier_de_mot_de_passe --new-vault-password-file nouveau_fichier_de_mot_de_passe fichier_vault` : remplace le fichier de mot de passe d'un fichier vault déjà chiffré par un autre

## Links
- More information: https://docs.ansible.com/projects/ansible/latest/vault_guide/index.html
