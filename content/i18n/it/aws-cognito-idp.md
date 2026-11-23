# aws cognito-idp

Configura un pool di utenti Amazon Cognito, i suoi utenti e gruppi e li autentica.

## Usage
- `aws cognito-idp create-user-pool --pool-name nome` : crea un nuovo pool di utenti Cognito
- `aws cognito-idp list-user-pools --max-results 10` : elenca tutti i pool di utenti
- `aws cognito-idp delete-user-pool --user-pool-id id_user_pool` : elimina un pool di utenti specifico
- `aws cognito-idp admin-create-user --username nome_utente --user-pool-id id_user_pool` : crea un utente in un pool specifico
- `aws cognito-idp list-users --user-pool-id id_user_pool` : elenca gli utenti di un pool specifico
- `aws cognito-idp admin-delete-user --username nome_utente --user-pool-id id_user_pool` : elimina un utente da un pool specifico

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/cognito-idp/
