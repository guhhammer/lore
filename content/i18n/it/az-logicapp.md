# az logicapp

Gestisce le Logic Apps nei servizi cloud Azure. Parte di `azure-cli` (noto anche come `az`).

## Usage
- `az logicapp create -n nome -g gruppo_risorse -s account_archiviazione` : crea una logic app
- `az logicapp delete -n nome -g gruppo_risorse` : elimina una logic app
- `az logicapp list -g gruppo_risorse` : elenca le logic app
- `az logicapp restart -n nome -g gruppo_risorse` : riavvia una logic app
- `az logicapp start -n nome -g gruppo_risorse` : avvia una logic app
- `az logicapp stop -n nome -g gruppo_risorse` : ferma una logic app

## Links
- More information: https://learn.microsoft.com/cli/azure/logicapp
