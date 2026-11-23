# aws acm

AWS Certificate Manager.

## Usage
- `aws acm import-certificate --certificate-arn certificado_arn --certificate certificado --private-key clave_privada --certificate-chain certificado_cadena` : importa un certificado
- `aws acm list-certificates` : lista certificados
- `aws acm describe-certificate --certificate-arn certificado_arn` : describe un certificado
- `aws acm request-certificate --domain-name nombre_dominio --validation-method método_de_validación` : solicita un certificado
- `aws acm delete-certificate --certificate-arn certificate_arn` : elimina un certificado
- `aws acm list-certificates --certificate-statuses estado` : lista validaciones de certificados
- `aws acm get-certificate --certificate-arn certificado_arn` : obtén detalles del certificado
- `aws acm update-certificate-options --certificate-arn certificado_arn --options opciones` : actualiza las opciones del certificado

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/acm/
