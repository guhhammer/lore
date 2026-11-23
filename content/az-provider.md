# az provider

Manage resource providers. Part of `azure-cli` (also known as `az`).

## Usage
- `az provider register -n Microsoft.PolicyInsights` : register a provider
- `az provider unregister -n Microsoft.Automation` : unregister a provider
- `az provider list` : list all providers for a subscription
- `az provider show -n Microsoft.Storage` : show information about a specific provider
- `az provider list --query "[?namespace=='Microsoft.Network'].resourceTypes[].resourceType"` : list all resource types for a specific provider

## Categories
cloud, network

## Links
- More information: https://learn.microsoft.com/cli/azure/provider
- tldr page: https://tldr.inbrowser.app/pages/common/az-provider
