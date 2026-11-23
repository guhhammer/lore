# aws-google-auth

Outil en ligne de commande pour obtenir des credentials AWS temporaire (STS) en utilisant Google Apps comme un fournisseur de fédération (SSO).

## Usage
- `aws-google-auth -u exemple@example.com -I $GOOGLE_IDP_ID -S $GOOGLE_SP_ID -d 3600` : connecte l'utilisateur avec le SSO Google en utilisant les identifiants IDP et SP et donne une durée de vie d'une heure à la connection
- `aws-google-auth -u exemple@example.com -I $GOOGLE_IDP_ID -S $GOOGLE_SP_ID -d 3600 -a` : connecte l'utilisateur en lui demandant quel rôle utiliser (dans le cas où il y a plusieurs rôles SAML)
- `aws-google-auth -u exemple@example.com -I $GOOGLE_IDP_ID -S $GOOGLE_SP_ID -d 3600 -a --resolve-aliases` : résous les alias pour des comptes AWS
- `aws-google-auth -h` : affiche l'aide

## Links
- More information: https://github.com/cevoaustralia/aws-google-auth
