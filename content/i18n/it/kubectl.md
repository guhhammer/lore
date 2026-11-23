# kubectl

Interfaccia interattiva da linea di comando per eseguire comandi sui clusters Kubernetes. Alcuni comandi aggiuntivi, come `run`, hanno la propria documentazione.

## Usage
- `kubectl get pods|service|deployment|ingress|... -o wide` : elenca le informazioni su una risorsa in maniera dettagliata
- `kubectl label pods nome unhealthy=true` : aggiorna il pod specificato con l'etichetta 'unhealthy' e il valore 'true'
- `kubectl get all` : elenca tutte le risorse
- `kubectl top pods|nodes` : mostra l'utilizzo delle risorse (CPU/Memory/Storage) di nodi o pods
- `kubectl cluster-info` : mostra l'indirizzo del master e i servizi del cluster
- `kubectl explain pods.spec.containers` : mostra la spiegazione di un campo specifico
- `kubectl logs nome_pod` : mostra i logs di un container in un pod o in una risorsa specificata
- `kubectl exec nome_pod -- ls /` : esegue un commando in un pod esistente

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/
