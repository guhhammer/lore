# kubectl get

Abfragen von Kubernetes Resourcen und Objekten.

## Usage
- `kubectl get namespaces` : zeige alle Namespaces im Cluster an
- `kubectl get nodes -n namespace` : frage alle Nodes in einem bestimmten namespace ab
- `kubectl get pods -n namespace` : frage alle Pods in einem bestimmten namespace ab
- `kubectl get deployments -n namespace` : frage alle Deployments in einem bestimmten namespace ab
- `kubectl get services -n namespace` : frage alle Services in einem bestimmten namespace ab
- `kubectl get all -n namespace` : frage alle Resourcen in einem bestimmten namespace ab
- `kubectl get -f pfad/zu/manifest.yaml` : frage alle Ressourcen ab, die in einer YAML Datei definiert sind

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_get/
