# age

Un outil de chiffrement de fichiers simple, moderne et sécurisé.

## Usage
- `age -p -o chemin/vers/fichier_chiffré.age chemin/vers/fichier_non_chiffré` : génère un fichier chiffré déchiffrable avec une phrase de passe
- `age -r clé_publique -o chemin/vers/fichier_chiffré.age chemin/vers/fichier_non_chiffré` : chiffre un fichier avec une ou plusieurs clés publiques entrées comme des littéraux (répéter `--recipient` pour plusieurs clés)
- `age -R chemin/vers/fichier_destinataires.txt -o chemin/vers/fichier_chiffré.age chemin/vers/fichier_non_chiffré` : chiffre un fichier avec les clés publiques spécifiées dans un fichier de destinataires (une par ligne)
- `age -d -o chemin/vers/fichier_déchiffré chemin/vers/fichier_chiffré.age` : déchiffre un fichier avec une phrase de passe
- `age -d -i chemin/vers/fichier_clé_privée -o chemin/vers/fichier_déchiffré chemin/vers/fichier_chiffré.age` : déchiffre un fichier avec un fichier de clé privée

## Links
- More information: https://github.com/FiloSottile/age#usage
