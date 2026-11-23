# aws acm

AWS Certificate Manager.

## Usage
- `aws acm import-certificate --certificate-arn arn_certificato --certificate certificato --private-key chiave_privata --certificate-chain catena_certificati` : importa un certificato
- `aws acm list-certificates` : elenca i certificati
- `aws acm describe-certificate --certificate-arn arn_certificato` : descrive un certificato
- `aws acm request-certificate --domain-name nome_dom --validation-method metodo_validazione` : richiede un certificato
- `aws acm delete-certificate --certificate-arn arn_certificato` : elimina un certificato
- `aws acm list-certificates --certificate-statuses stato` : elenca le convalide dei certificati (usa `--certificate-statuses stato` con `list-certificates`)
- `aws acm get-certificate --certificate-arn arn_certificato` : ottiene i dettagli di un certificato
- `aws acm update-certificate-options --certificate-arn arn_certificato --options opzioni` : aggiorna le opzioni di un certificato

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/acm/
