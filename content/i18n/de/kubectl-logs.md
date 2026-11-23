# kubectl logs

Logs für Container in einem Pod anzeigen.

## Usage
- `kubectl logs pod_name` : zeige Logs für einen Einzelcontainer-Pod an
- `kubectl logs --container container_name pod_name` : zeige Logs für einen bestimmten Container in einem Pod an
- `kubectl logs --all-containers=true pod_name` : zeige Logs für alle Container in einem Pod an
- `kubectl logs --follow pod_name` : folge den Pod-Logs (stream)
- `kubectl logs --since=relative_time pod_name` : zeige Pod-Logs die neuer einer relativen Zeit sind `10s`, `5m`, or `1h`
- `kubectl logs --tail=10 pod_name` : zeige die 10 neuesten Logzeilen in einem Pod

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_logs/
