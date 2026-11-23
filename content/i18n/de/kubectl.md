# kubectl

Befehlszeilenschnittstelle zur Ausführung von Befehlen gegen Kubernetes-Cluster. Einige Unterbefehle wie `kubectl run` haben ihre eigene Dokumentation zur Verwendung.

## Usage
- `kubectl get pods|service|deployment|ingress|... -o wide` : liste Informationen über eine Ressource mit weiteren Details auf
- `kubectl label pods name unhealthy=true` : aktualisiere die angegebenen Pods mit dem Label 'unhealthy' und dem Wert 'true'
- `kubectl get all` : liste alle Ressourcen aller Typen auf
- `kubectl top pods|nodes` : zeige die Ressourcennutzung (CPU/Memory/Storage) von Knoten oder Pods
- `kubectl cluster-info` : zeige die Adresse der Master- und Clusterdienste
- `kubectl explain pods.spec.containers` : zeige eine Erklärung zu einem bestimmten Feld an
- `kubectl logs pod_name` : zeige Logs für einen Container in einem Pod oder einer bestimmten Ressource
- `kubectl exec pod_name -- ls /` : führe einen Befehl in einem bestehenden Pod aus

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/
