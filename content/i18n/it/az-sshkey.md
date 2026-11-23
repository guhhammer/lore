# az sshkey

Gestisce le chiavi pubbliche SSH con le macchine virtuali. Parte di `azure-cli` (noto anche come `az`).

## Usage
- `az sshkey create --name nome -g gruppo_risorse` : crea una nuova chiave SSH
- `az sshkey create --name nome -g gruppo_risorse --public-key "@percorso/verso/chiave.pub"` : carica una chiave SSH esistente
- `az sshkey list` : elenca tutte le chiavi pubbliche SSH
- `az sshkey show --name nome -g gruppo_risorse` : mostra informazioni su una chiave pubblica SSH

## Links
- More information: https://learn.microsoft.com/cli/azure/sshkey
