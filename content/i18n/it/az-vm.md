# az vm

Gestisce le macchine virtuali in Azure. Parte di `azure-cli` (noto anche come `az`).

## Usage
- `az vm list --output table` : visualizza una tabella delle Virtual Machines disponibili
- `az vm create -g rg -n nome_vm --image UbuntuLTS --admin-user azureuser --generate-ssh-keys` : crea una macchina virtuale usando l'immagine Ubuntu predefinita e genera chiavi SSH
- `az vm stop -g rg -n nome_vm` : ferma una Virtual Machine
- `az vm deallocate -g rg -n nome_vm` : dealloca una Virtual Machine
- `az vm start -g rg -n nome_vm` : avvia una Virtual Machine
- `az vm restart -g rg -n nome_vm` : riavvia una Virtual Machine
- `az vm image list` : elenca le immagini VM disponibili nel marketplace Azure

## Links
- More information: https://learn.microsoft.com/cli/azure/vm
