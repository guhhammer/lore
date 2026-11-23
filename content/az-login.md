# az login

Log in to Azure. Part of `azure-cli` (also known as `az`).

## Usage
- `az login` : log in interactively
- `az login --service-principal -u http://azure-cli-service-principal -p secret -t someone.onmicrosoft.com` : log in with a service principal using a client secret
- `az login --service-principal -u http://azure-cli-service-principal -p path/to/cert.pem -t someone.onmicrosoft.com` : log in with a service principal using a client certificate
- `az login -i` : log in using a VM's system assigned identity
- `az login -i -u /subscriptions/subscription_id/resourcegroups/my_rg/providers/Microsoft.ManagedIdentity/userAssignedIdentities/my_id` : log in using a VM's user assigned identity

## Categories
users, cloud, monitoring

## Links
- More information: https://learn.microsoft.com/cli/azure/reference-index#az-login
- tldr page: https://tldr.inbrowser.app/pages/common/az-login
