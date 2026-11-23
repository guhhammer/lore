# aws iam

Interact with Identity and Access Management (IAM), a web service for securely controlling access to AWS services.

## Usage
- `aws iam list-users` : list users
- `aws iam list-policies` : list policies
- `aws iam list-groups` : list groups
- `aws iam get-group --group-name group_name` : get users in a group
- `aws iam get-policy --policy-arn arn:aws:iam::aws:policy/policy_name` : describe an IAM policy
- `aws iam list-access-keys` : list access keys
- `aws iam list-access-keys --user-name user_name` : list access keys for a specific user
- `aws iam help` : display help

## Categories
cloud, users, system

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/iam/
- tldr page: https://tldr.inbrowser.app/pages/common/aws-iam
