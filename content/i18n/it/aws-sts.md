# aws sts

Security Token Service (STS) permette di richiedere credenziali temporanee per utenti (IAM) o utenti federati.

## Usage
- `aws sts assume-role --role-arn arn_ruolo_aws` : ottieni credenziali di sicurezza temporanee per accedere a risorse AWS specifiche
- `aws sts get-caller-identity` : ottieni un utente IAM o ruolo le cui credenziali sono utilizzate per chiamare l'operazione

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/sts/
