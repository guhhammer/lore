# az acr

Gestisce i registri privati con Azure Container Registries. Parte di `azure-cli` (noto anche come `az`).

## Usage
- `az acr create -n nome_registro -g gruppo_risorse --sku sku` : crea un registro container gestito
- `az acr login -n nome_registro` : accedi a un registro
- `docker tag nome_immagine nome_registro.azurecr.io/nome_immagine:tag` : etichetta un'immagine locale per ACR
- `docker push nome_registro.azurecr.io/nome_immagine:tag` : carica un'immagine su un registro
- `docker pull nome_registro.azurecr.io/nome_immagine:tag` : scarica un'immagine da un registro
- `az acr repository delete -n nome_registro --repository nome_immagine:tag` : elimina un'immagine da un registro
- `az acr delete -n nome_registro -g gruppo_risorse -y` : elimina un registro container gestito
- `az acr repository list -n nome_registro --output table` : elenca le immagini all'interno di un registro

## Links
- More information: https://learn.microsoft.com/cli/azure/acr
