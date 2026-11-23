# az redis

Gestisce le cache Redis. Parte di `azure-cli` (noto anche come `az`).

## Usage
- `az redis create --location posizione -n nome -g gruppo_risorse --sku Basic|Premium|Standard --vm-size c0|c1|c2|c3|c4|c5|c6|p1|p2|p3|p4|p5` : crea una nuova istanza di cache Redis
- `az redis update -n nome -g gruppo_risorse --sku Basic|Premium|Standard --vm-size c0|c1|c2|c3|c4|c5|c6|p1|p2|p3|p4|p5` : aggiorna una cache Redis
- `az redis export --container container --file-format formato_file -n nome --prefix prefisso -g gruppo_risorse` : esporta i dati memorizzati in una cache Redis
- `az redis delete -n nome -g gruppo_risorse -y` : elimina una cache Redis

## Links
- More information: https://learn.microsoft.com/cli/azure/redis
