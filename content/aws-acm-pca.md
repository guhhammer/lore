# aws acm-pca

AWS Certificate Manager Private Certificate Authority.

## Usage
- `aws acm-pca create-certificate-authority --certificate-authority-configuration ca_config --idempotency-token token --permanent-deletion-time-in-days number` : create a private certificate authority
- `aws acm-pca describe-certificate-authority --certificate-authority-arn ca_arn` : describe a private certificate authority
- `aws acm-pca list-certificate-authorities` : list private certificate authorities
- `aws acm-pca update-certificate-authority --certificate-authority-arn ca_arn --certificate-authority-configuration ca_config --status status` : update a certificate authority
- `aws acm-pca delete-certificate-authority --certificate-authority-arn ca_arn` : delete a private certificate authority
- `aws acm-pca issue-certificate --certificate-authority-arn ca_arn --certificate-signing-request cert_signing_request --signing-algorithm algorithm --validity validity` : issue a certificate
- `aws acm-pca revoke-certificate --certificate-authority-arn ca_arn --certificate-serial serial --reason reason` : revoke a certificate
- `aws acm-pca get-certificate --certificate-authority-arn ca_arn --certificate-arn cert_arn` : get certificate details

## Categories
security, cloud

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/acm-pca/
- tldr page: https://tldr.inbrowser.app/pages/common/aws-acm-pca
