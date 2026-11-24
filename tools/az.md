# az

az is the Azure command line. This page covers signing in, subscriptions, resource groups, virtual machines, storage and web apps.

## Tasks
### Sign in and choose a subscription
- `az login` : sign in through the browser
- `az login --use-device-code` : sign in from a machine without a browser
- `az account list --output table` : your subscriptions
- `az account set --subscription NAME` : choose one
- `az account show` : the one in use
- `az logout` : sign out
### Resource groups
- `az group list --output table` : resource groups
- `az group create --name NAME --location westeurope` : make one
- `az group delete --name NAME` : delete a group and everything in it (deletes it)
- `az resource list --resource-group NAME --output table` : what is inside
### Virtual machines
- `az vm list --output table` : virtual machines
- `az vm create --resource-group NAME --name VM --image Ubuntu2204 --admin-username USER --generate-ssh-keys` : make an Ubuntu VM
- `az vm start --resource-group NAME --name VM` : start it
- `az vm deallocate --resource-group NAME --name VM` : stop it and stop the billing for compute
- `az vm list-ip-addresses --name VM` : its IP addresses
- `az vm open-port --resource-group NAME --name VM --port 80` : open a port
### Storage
- `az storage account list --output table` : storage accounts
- `az storage blob upload --account-name ACCOUNT --container-name NAME --file FILE --name BLOB --auth-mode login` : upload a file
- `az storage blob list --account-name ACCOUNT --container-name NAME --auth-mode login --output table` : list files
### Apps and containers
- `az webapp list --output table` : web apps
- `az webapp log tail --name NAME --resource-group NAME` : follow an app's logs
- `az aks get-credentials --resource-group NAME --name CLUSTER` : point kubectl at an AKS cluster
- `az acr login --name REGISTRY` : sign Docker in to a registry
### Output and help
- `az ... --output table` : readable table (also json, yaml, tsv)
- `az ... --query "[].name"` : pick fields with a JMESPath expression
- `az find "TEXT"` : find commands for a task

## Notes
- Billing follows the subscription: check `az account show` before creating things.
- `az group delete` removes everything inside the group and cannot be undone.
- Stopping a VM from inside the system keeps billing compute; use `deallocate`.
- Install from the official page in Links.

## See also
- terraform
- aws
- gcloud
- kubectl

## Links
- Reference: https://learn.microsoft.com/en-us/cli/azure/
- Install: https://learn.microsoft.com/en-us/cli/azure/install-azure-cli

## Categories
cloud
