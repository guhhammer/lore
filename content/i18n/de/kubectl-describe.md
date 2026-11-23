# kubectl describe

Details von Kubernetes-Objekten und -Ressourcen anzeigen.

## Usage
- `kubectl describe po -n namespace` : zeige Details zu Pods in einem bestimmten namespace an
- `kubectl describe no -n namespace` : zeige Details zu Nodes in einem bestimmten namespace an
- `kubectl describe po pod_name -n namespace` : zeige Details zu einem bestimmten Pod in einem bestimmten namespace an
- `kubectl describe no node_name -n namespace` : zeige Details zu einer bestimmten Node in einem bestimmten namespace an
- `kubectl describe -f pfad/zu/manifest.yaml` : zeige Details zu Ressourcen, die in einer YAML Datei definiert sind, an

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_describe/
