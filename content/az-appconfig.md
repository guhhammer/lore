# az appconfig

Manage App configurations on Azure. Part of `azure-cli` (also known as `az`).

## Usage
- `az appconfig create -n name -g group_name -l location` : create an App Configuration
- `az appconfig delete -g rg_name -n appconfig_name` : delete a specific App Configuration
- `az appconfig list` : list all App Configurations under the current subscription
- `az appconfig list -g rg_name` : list all App Configurations under a specific resource group
- `az appconfig show -n appconfig_name` : show properties of an App Configuration
- `az appconfig update -g rg_name -n appconfig_name` : update a specific App Configuration

## Categories
cloud

## Links
- More information: https://learn.microsoft.com/cli/azure/appconfig
- tldr page: https://tldr.inbrowser.app/pages/common/az-appconfig
