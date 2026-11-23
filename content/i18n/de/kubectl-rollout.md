# kubectl rollout

Verwalten des Rollouts einer Kubernetes-Ressource (deployments, daemonsets, and statefulsets).

## Usage
- `kubectl rollout restart resource_type/resource_name` : starte einen rollenden Neustart einer Ressource
- `kubectl rollout status resource_type/resource_name` : überwache den fortlaufenden Aktualisierungsstatus einer Ressource
- `kubectl rollout undo resource_type/resource_name` : setze eine Ressource auf die vorherige Version zurück
- `kubectl rollout history resource_type/resource_name` : zeige den Rollout-Verlauf einer Ressource an

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_rollout/
