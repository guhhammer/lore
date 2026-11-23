# az account

Manage Azure subscription information. Part of `azure-cli` (also known as `az`).

## Usage
- `az account list` : list all subscriptions for the logged in account
- `az account set -s subscription_id` : set a `subscription` to be the currently active subscription
- `az account list-locations` : list supported regions for the currently active subscription
- `az account get-access-token --resource-type ms-graph` : print an access token to be used with `MS Graph API`
- `az account show -o json|tsv|table|yaml` : print details of the currently active subscription in a specific format

## Categories
users, cloud

## Links
- More information: https://learn.microsoft.com/cli/azure/account
- tldr page: https://tldr.inbrowser.app/pages/common/az-account
