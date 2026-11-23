# az container

Gestisce le istanze Azure Container. Parte di `azure-cli` (noto anche come `az`).

## Usage
- `az container create -g gruppo_risorse -n nome --image nome_immagine -os windows|linux --cpu numero_core_cpu --memory memoria_GB` : crea un contenitore in un gruppo di contenitori
- `az container exec -g gruppo_risorse -n nome_gruppo_contenitori --exec-command "comando"` : esegue un comando da un contenitore in esecuzione di un gruppo di contenitori
- `az container logs -n nome -g gruppo_risorse` : esamina i log di un contenitore in un gruppo di contenitori
- `az container show -n nome -g gruppo_risorse` : ottiene i dettagli di un gruppo di contenitori
- `az container start -n nome -g gruppo_risorse` : avvia tutti i contenitori in un gruppo di contenitori
- `az container stop -n nome -g gruppo_risorse` : ferma tutti i contenitori in un gruppo di contenitori
- `az container delete -n nome -g gruppo_risorse` : elimina un gruppo di contenitori

## Links
- More information: https://learn.microsoft.com/cli/azure/container
