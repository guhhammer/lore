# az appconfig

Gestisce le configurazioni App su Azure. Parte di `azure-cli` (noto anche come `az`).

## Usage
- `az appconfig create -n nome -g nome_gruppo -l posizione` : crea una configurazione App
- `az appconfig delete -g nome_rg -n nome_appconfig` : elimina una configurazione App specifica
- `az appconfig list` : elenca tutte le configurazioni App sotto la sottoscrizione corrente
- `az appconfig list -g nome_rg` : elenca tutte le configurazioni App sotto un gruppo di risorse specifico
- `az appconfig show -n nome_appconfig` : mostra le proprietà di una configurazione App
- `az appconfig update -g nome_rg -n nome_appconfig` : aggiorna una configurazione App specifica

## Links
- More information: https://learn.microsoft.com/cli/azure/appconfig
