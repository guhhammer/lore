# aws-google-auth

Linha de comando para obter credenciais (STS) temporárias AWS usando o Google Apps como um provedor (Single Sign-On) federado.

## Usage
- `aws-google-auth -u exemplo@example.com -I $GOOGLE_IDP_ID -S $GOOGLE_SP_ID -d 3600` : loga com o Google SSO usando identificadores IDP e SP e cria credenciais com duração de uma hora
- `aws-google-auth -u examplo@example.com -I $GOOGLE_IDP_ID -S $GOOGLE_SP_ID -d 3600 -a` : loga perguntando (asking) qual papel usar (no caso de diversos papeis SAML disponíveis)
- `aws-google-auth -u examplo@example.com -I $GOOGLE_IDP_ID -S $GOOGLE_SP_ID -d 3600 -a --resolve-aliases` : resolve aliases para contas AWS
- `aws-google-auth -h` : mostra ajuda

## Links
- More information: https://github.com/cevoaustralia/aws-google-auth
