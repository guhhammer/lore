# az image

Gestisce le immagini personalizzate di Virtual Machine in Azure. Parte di `azure-cli` (noto anche come `az`).

## Usage
- `az image list -g gruppo_risorse` : elenca le immagini personalizzate sotto un gruppo di risorse
- `az image create -g gruppo_risorse -n nome --os-type windows|linux --source sorgente_disco_os` : crea un'immagine personalizzata da dischi gestiti o snapshot
- `az image delete -n nome -g gruppo_risorse` : elimina un'immagine personalizzata
- `az image show -n nome -g gruppo_risorse` : mostra i dettagli di un'immagine personalizzata
- `az image update -n nome -g gruppo_risorse --set proprietà=valore` : aggiorna le immagini personalizzate

## Links
- More information: https://learn.microsoft.com/cli/azure/image
