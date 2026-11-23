# az login

Log in bij Azure. Onderdeel van `azure-cli` (ook bekend als `az`).

## Usage
- `az login` : log interactief in
- `az login --service-principal -u http://azure-cli-service-principal -p secret -t iemand.onmicrosoft.com` : log in als een service principal met behulp van een client secret
- `az login --service-principal -u http://azure-cli-service-principal -p pad/naar/cert.pem -t iemand.onmicrosoft.com` : log in als een service principal met behulp van een client certificaat
- `az login -i` : log in met de door het systeem toegewezen identiteit van een VM
- `az login -i -u /subscriptions/abonnement_id/resourcegroups/mijn_rg/providers/Microsoft.ManagedIdentity/userAssignedIdentities/mijn_id` : log in met de door de gebruiker toegewezen identiteit van een VM

## Links
- More information: https://learn.microsoft.com/cli/azure/reference-index#az-login
