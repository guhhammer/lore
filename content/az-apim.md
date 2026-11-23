# az apim

Manage Azure API Management services. Part of `azure-cli` (also known as `az`).

## Usage
- `az apim list -g resource_group` : list API Management services within a resource group
- `az apim create -n name -g resource_group --publisher-email email --publisher-name name` : create an API Management service instance
- `az apim delete -n name -g resource_group` : delete an API Management service
- `az apim show -n name -g resource_group` : show details of an API Management service instance
- `az apim update -n name -g resource_group` : update an API Management service instance

## Categories
system, cloud

## Links
- More information: https://learn.microsoft.com/cli/azure/apim
- tldr page: https://tldr.inbrowser.app/pages/common/az-apim
