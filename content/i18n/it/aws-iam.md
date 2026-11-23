# aws iam

Interagisce con Identity and Access Management (IAM), un servizio web per controllare in modo sicuro l'accesso ai servizi AWS.

## Usage
- `aws iam list-users` : elenca gli utenti
- `aws iam list-policies` : elenca le policy
- `aws iam list-groups` : elenca i gruppi
- `aws iam get-group --group-name nome_gruppo` : ottieni gli utenti in un gruppo
- `aws iam get-policy --policy-arn arn:aws:iam::aws:policy/nome_policy` : descrivi una policy IAM
- `aws iam list-access-keys` : elenca le chiavi di accesso
- `aws iam list-access-keys --user-name nome_utente` : elenca le chiavi di accesso per un utente specifico
- `aws iam help` : mostra l'aiuto

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/iam/
