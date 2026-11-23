# helm

Helm è un gestore di pacchetti per Kubernetes. Alcuni comandi aggiuntivi, come `install`, hanno la propria documentazione.

## Usage
- `helm create nome_chart` : crea una helm chart
- `helm repo add nome_repository` : aggiungi un nuovo repository helm
- `helm repo ls` : elenca i repositories helm
- `helm repo up` : aggiorna i repositories helm
- `helm repo rm nome_repository` : cancella un repository helm
- `helm install nome_chart nome_repository/nome_chart` : installa una helm chart
- `helm get nome_chart_rilasciata` : scarica una helm chart sottoforma di archivio `.tar`
- `helm dep up` : aggiorna le dipendenze helm

## Links
- More information: https://helm.sh/docs/helm/
