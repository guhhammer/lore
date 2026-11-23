# az login

Melden Sie sich bei Azure an. Teil von `azure-cli` (auch bekannt als `az`).

## Usage
- `az login` : melden Sie sich interaktiv an
- `az login --service-principal -u http://azure-cli-service-principal -p secret -t someone.onmicrosoft.com` : melden Sie sich mit einem Dienstprinzipal mit dem geheimen Clientschlüssel an
- `az login --service-principal -u http://azure-cli-service-principal -p path/to/cert.pem -t someone.onmicrosoft.com` : melden Sie sich mit einem Dienstprinzipal mithilfe des Clientzertifikats an
- `az login -i` : melden Sie sich mit der vom System zugewiesenen verwalteten Identität eines virtuellen Computers an
- `az login -i -u /subscriptions/subscription_id/resourcegroups/my_rg/providers/Microsoft.ManagedIdentity/userAssignedIdentities/my_id` : melden Sie sich mit der vom Benutzer zugewiesenen verwalteten Identität eines virtuellen Computers an

## Links
- More information: https://learn.microsoft.com/cli/azure/reference-index#az-login
