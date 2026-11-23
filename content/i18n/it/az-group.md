# az group

Gestisce i gruppi di risorse e le distribuzioni di template. Parte di `azure-cli` (noto anche come `az`).

## Usage
- `az group create -n nome -l posizione` : crea un nuovo gruppo di risorse
- `az group exists -n nome` : verifica se un gruppo di risorse esiste
- `az group delete -n nome` : elimina un gruppo di risorse
- `az group wait -n nome --created|deleted|exists|updated` : aspetta fino a quando una condizione del gruppo di risorse è soddisfatta

## Links
- More information: https://learn.microsoft.com/cli/azure/group
