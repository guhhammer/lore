# aws cognito-idp

Administra el grupo de usuarios de Amazon Cognito y sus usuarios y grupos utilizando la CLI.

## Usage
- `aws cognito-idp create-user-pool --pool-name nombre` : crea un nuevo grupo de usuarios de Cognito
- `aws cognito-idp list-user-pools --max-results 10` : lista todos los grupos de usuarios
- `aws cognito-idp delete-user-pool --user-pool-id identificador_de_pool` : elimina un grupo de usuarios específico
- `aws cognito-idp admin-create-user --username usuario --user-pool-id identificador_de_pool` : crea un usuario en un grupo específico
- `aws cognito-idp list-users --user-pool-id identificador_de_pool` : lista los usuarios de un pool específico
- `aws cognito-idp admin-delete-user --username usuario --user-pool-id identificador_de_pool` : elimina un usuario de un grupo específico

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/cognito-idp/
