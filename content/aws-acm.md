# aws acm

AWS Certificate Manager.

## Usage
- `aws acm import-certificate --certificate-arn certificate_arn --certificate certificate --private-key private_key --certificate-chain certificate_chain` : import a certificate
- `aws acm list-certificates` : list certificates
- `aws acm describe-certificate --certificate-arn certificate_arn` : describe a certificate
- `aws acm request-certificate --domain-name domain_name --validation-method validation_method` : request a certificate
- `aws acm delete-certificate --certificate-arn certificate_arn` : delete a certificate
- `aws acm list-certificates --certificate-statuses status` : list certificate validations
- `aws acm get-certificate --certificate-arn certificate_arn` : get certificate details
- `aws acm update-certificate-options --certificate-arn certificate_arn --options options` : update certificate options

## Categories
security, cloud

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/acm/
- tldr page: https://tldr.inbrowser.app/pages/common/aws-acm
