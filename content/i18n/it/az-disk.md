# az disk

Gestisce i dischi gestiti Azure. Parte di `azure-cli` (noto anche come `az`).

## Usage
- `az disk create -g gruppo_risorse -n nome_disco -z dimensione_gb` : crea un disco gestito
- `az disk list -g gruppo_risorse` : elenca i dischi gestiti in un gruppo di risorse
- `az disk delete -g gruppo_risorse -n nome_disco` : elimina un disco gestito
- `az disk grant-access -g gruppo_risorse -n nome_disco --access Read|Write --duration-in-seconds secondi` : concede accesso di lettura o scrittura a un disco gestito (per esportazione)
- `az disk update -g gruppo_risorse -n nome_disco -z nuova_dimensione_gb` : aggiorna la dimensione del disco

## Links
- More information: https://learn.microsoft.com/cli/azure/disk
