# az aks

Gestisce i cluster Azure Kubernetes Service (AKS). Parte di `azure-cli` (noto anche come `az`).

## Usage
- `az aks list -g gruppo_risorse` : elenca i cluster AKS
- `az aks create -g gruppo_risorse -n nome -c conteggio --node-vm-size dimensione` : crea un nuovo cluster AKS
- `az aks delete -g gruppo_risorse -n nome` : elimina un cluster AKS
- `az aks get-credentials -g gruppo_risorse -n nome` : ottiene le credenziali di accesso per un cluster AKS
- `az aks get-upgrades -g gruppo_risorse -n nome` : ottiene le versioni di upgrade disponibili per un cluster AKS

## Links
- More information: https://learn.microsoft.com/cli/azure/aks
