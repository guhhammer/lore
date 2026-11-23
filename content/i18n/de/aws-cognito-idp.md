# aws cognito-idp

Verwalten des Amazon Cognito-Benutzerpools, seiner Benutzer und Gruppen mit der CLI.

## Usage
- `aws cognito-idp create-user-pool --pool-name name` : erstelle einen neuen Cognito-Benutzerpool
- `aws cognito-idp list-user-pools --max-results 10` : liste alle Bentzerpools auf
- `aws cognito-idp delete-user-pool --user-pool-id benutzerpool_id` : lösche einen bestimmten Benutzerpool
- `aws cognito-idp admin-create-user --username benutzer --user-pool-id benutzerpool_id` : erstelle einen Benutzer in einem bestimmten Pool
- `aws cognito-idp list-users --user-pool-id benutzerpool_id` : liste die Benutzer eines bestimmten Pool auf
- `aws cognito-idp admin-delete-user --username benutzer --user-pool-id benutzerpool_id` : lösche einen Benutzer aus einem bestimmten Pool

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/cognito-idp/
