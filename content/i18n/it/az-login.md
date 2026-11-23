# az login

Accedi ad Azure. Parte di `azure-cli` (noto anche come `az`).

## Usage
- `az login` : accedi in modo interattivo
- `az login --service-principal -u http://azure-cli-service-principal -p segreto -t someone.onmicrosoft.com` : accedi con un principal del servizio usando un segreto client
- `az login --service-principal -u http://azure-cli-service-principal -p percorso/verso/cert.pem -t someone.onmicrosoft.com` : accedi con un principal del servizio usando un certificato client
- `az login -i` : accedi usando l'identità assegnata dal sistema di una VM
- `az login -i -u /subscriptions/id_sottoscrizione/resourcegroups/mio_rg/providers/Microsoft.ManagedIdentity/userAssignedIdentities/mio_id` : accedi usando l'identità assegnata dall'utente di una VM

## Links
- More information: https://learn.microsoft.com/cli/azure/reference-index#az-login
