# aws-google-auth

Acquire AWS temporary (STS) credentials using Google Apps as a federated (Single Sign-On) provider.

## Usage
- `aws-google-auth -u example@example.com -I $GOOGLE_IDP_ID -S $GOOGLE_SP_ID -d 3600` : log in with Google SSO using the specified username IDP and SP identifiers and set the credentials duration to one hour
- `aws-google-auth -u example@example.com -I $GOOGLE_IDP_ID -S $GOOGLE_SP_ID -d 3600 -a` : log in asking which role to use (in case of several available SAML roles)
- `aws-google-auth -u example@example.com -I $GOOGLE_IDP_ID -S $GOOGLE_SP_ID -d 3600 -a --resolve-aliases` : resolve aliases for AWS accounts
- `aws-google-auth -h` : display help

## Categories
cloud

## Links
- More information: https://github.com/cevoaustralia/aws-google-auth
- tldr page: https://tldr.inbrowser.app/pages/common/aws-google-auth
