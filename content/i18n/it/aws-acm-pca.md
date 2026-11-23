# aws acm-pca

Autorità di certificazione privata AWS Certificate Manager.

## Usage
- `aws acm-pca create-certificate-authority --certificate-authority-configuration ca_config --idempotency-token token --permanent-deletion-time-in-days number` : crea un'autorità di certificazione privata
- `aws acm-pca describe-certificate-authority --certificate-authority-arn ca_arn` : descrivi un'autorità di certificazione privata
- `aws acm-pca list-certificate-authorities` : elenca le autorità di certificazione private
- `aws acm-pca update-certificate-authority --certificate-authority-arn ca_arn --certificate-authority-configuration ca_config --status status` : aggiorna un'autorità di certificazione
- `aws acm-pca delete-certificate-authority --certificate-authority-arn ca_arn` : elimina un'autorità di certificazione privata
- `aws acm-pca issue-certificate --certificate-authority-arn ca_arn --certificate-signing-request cert_signing_request --signing-algorithm algorithm --validity validity` : emetti un certificato
- `aws acm-pca revoke-certificate --certificate-authority-arn ca_arn --certificate-serial serial --reason reason` : revoca un certificato
- `aws acm-pca get-certificate --certificate-authority-arn ca_arn --certificate-arn cert_arn` : ottieni dettagli del certificato

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/acm-pca/
