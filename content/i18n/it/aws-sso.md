# aws sso

Gestisce l'accesso alle risorse AWS utilizzando credenziali Single Sign-On (SSO).

## Usage
- `aws sso login` : avvia sessione SSO e aggiorna i token di accesso. Richiede configurazione con `aws configure sso`
- `aws sso logout` : termina sessione SSO e cancella i token di accesso memorizzati nella cache
- `aws sso list-accounts --access-token token` : elenca tutti gli account AWS accessibili all'utente
- `aws sso list-account-roles --account-id account --access-token token` : elenca tutti i ruoli accessibili all'utente per un dato account AWS
- `aws sso get-role-credentials --account-id account --role-name ruolo --access-token token` : recupera credenziali a breve termine per un account specifico

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/sso/
