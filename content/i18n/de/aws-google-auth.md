# aws-google-auth

CLI, um temporäre AWS credentials (STS) über Google Apps als Single Sign-On Dienstleister zu erhalten.

## Usage
- `aws-google-auth -u beispiel@example.com -I $GOOGLE_IDP_ID -S $GOOGLE_SP_ID -d 3600` : einloggen mit Google SSO über IDP- und SP-Kennung für die Dauer einer Stunde
- `aws-google-auth -u beispiel@example.com -I $GOOGLE_IDP_ID -S $GOOGLE_SP_ID -d 3600 -a` : einloggen mit der Option eine Rolle auszuwählen (im Falle mehrerer verfügbarer SAML Rollen)
- `aws-google-auth -u beispiel@example.com -I $GOOGLE_IDP_ID -S $GOOGLE_SP_ID -d 3600 -a --resolve-aliases` : löse Aliasse von AWS Accounts auf
- `aws-google-auth -h` : zeige Hilfe an

## Links
- More information: https://github.com/cevoaustralia/aws-google-auth
