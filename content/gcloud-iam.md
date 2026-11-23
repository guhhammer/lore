# gcloud iam

Configure Identity and Access Management (IAM) preferences and service accounts.

## Usage
- `gcloud iam list-grantable-roles resource` : list IAM grantable roles for a resource
- `gcloud iam roles create role_name --organization|project organization|project_id --file path/to/role.yaml` : create a custom role for an organization or project
- `gcloud iam service-accounts create name` : create a service account for a project
- `gcloud iam service-accounts add-iam-policy-binding service_account_email --member member --role role` : add an IAM policy binding to a service account
- `gcloud iam service-accounts set-iam-policy service_account_email policy_file` : replace existing IAM policy binding
- `gcloud iam service-accounts keys list --iam-account service_account_email` : list a service account's keys

## Categories
cloud, system, users

## Links
- More information: https://docs.cloud.google.com/sdk/gcloud/reference/iam
- tldr page: https://tldr.inbrowser.app/pages/common/gcloud-iam
