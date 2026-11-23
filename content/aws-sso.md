# aws sso

Manage access to AWS resources using Single Sign-On (SSO) credentials.

## Usage
- `aws sso login` : start SSO session and refresh access tokens (requires setup using `aws configure sso`)
- `aws sso logout` : end SSO session and clear cached access tokens
- `aws sso list-accounts --access-token token` : list all AWS accounts accessible to the user
- `aws sso list-account-roles --account-id account --access-token token` : list all roles accessible to the user for a given AWS account
- `aws sso get-role-credentials --account-id account --role-name role --access-token token` : retrieve short-term credentials for a specific account

## Categories
cloud, users

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/sso/
- tldr page: https://tldr.inbrowser.app/pages/common/aws-sso
