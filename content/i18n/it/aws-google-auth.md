# aws-google-auth

Acquisisce credenziali AWS temporanee (STS) utilizzando Google Apps come fornitore federato (Single Sign-On).

## Usage
- `aws-google-auth -u example@example.com -I $GOOGLE_IDP_ID -S $GOOGLE_SP_ID -d 3600` : accedi con Google SSO utilizzando l'identificativo username IDP e SP specificati e imposta la durata delle credenziali a un'ora
- `aws-google-auth -u example@example.com -I $GOOGLE_IDP_ID -S $GOOGLE_SP_ID -d 3600 -a` : accedi chiedendo quale ruolo utilizzare (in caso di più ruoli SAML disponibili)
- `aws-google-auth -u example@example.com -I $GOOGLE_IDP_ID -S $GOOGLE_SP_ID -d 3600 -a --resolve-aliases` : risolve gli alias per gli account AWS
- `aws-google-auth -h` : mostra l'aiuto

## Links
- More information: https://github.com/cevoaustralia/aws-google-auth
