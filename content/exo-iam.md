# exo iam

Manage the Exoscale IAM service.

## Usage
- `exo iam role list` : list all of the IAM roles
- `exo iam api-key create api_key_name iam_role_name` : create a new API key
- `cat path/to/policy.json | exo iam role create iam_role_name --editable --policy -` : create a new IAM role
- `exo iam role show iam_role_name --policy -O json | jq .` : show the policy of an existing IAM role
- `cat path/to/policy.json | exo iam org-policy update -` : update the default Organization policy (the default Organization policy will be applied to all of the API keys within the Organization)

## Categories
system, text

## Links
- More information: https://community.exoscale.com/product/iam/
- tldr page: https://tldr.inbrowser.app/pages/common/exo-iam
