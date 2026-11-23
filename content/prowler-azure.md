# prowler azure

Assess Azure security best practices, perform audits, compliance checks, and generate reports.

## Usage
- `prowler azure --az-cli-auth` : run the default set of checks on the current Azure account using Azure CLI authentication
- `prowler azure --az-cli-auth --subscription-ids subscription_id1 subscription_id2 ...` : run checks for specific Azure subscriptions
- `prowler azure --sp-env-auth` : authenticate using a service principal via environment variables
- `prowler azure --browser-auth --tenant-id "XXXXXXXX"` : authenticate using browser login and specify a tenant ID
- `prowler azure --managed-identity-auth` : authenticate using a managed identity (e.g. for Azure VM)
- `prowler azure -s defender|iam|...` : run checks for selected Azure services
- `prowler azure -c storage_blob_public_access_level_is_disabled` : run a specific Azure check
- `prowler azure -e storage_blob_public_access_level_is_disabled --exclude-services defender|iam|...` : exclude specific checks or services

## See also
- prowler aws
- prowler gcp
- prowler kubernetes
- prowler m365
- prowler github

## Categories
cloud, system, users

## Links
- More information: https://docs.prowler.com/user-guide/cli/tutorials/misc
- tldr page: https://tldr.inbrowser.app/pages/common/prowler-azure
