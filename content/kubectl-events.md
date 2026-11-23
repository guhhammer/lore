# kubectl events

Display resource events.

## Usage
- `kubectl events` : show events in the default namespace
- `kubectl events -A` : show events in all namespaces
- `kubectl events -w -n namespace` : watch events in a specific namespace
- `kubectl events --for pod/pod_name -n namespace` : show events for a pod in a specific namespace
- `kubectl events --for resource/resource_name -n namespace` : show events for a resource in a specific namespace
- `kubectl events --types Warning,Normal` : show events for type `Warning` or `Normal`
- `kubectl events -o yaml` : output events in YAML format

## Categories
containers, desktop

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_events/
- tldr page: https://tldr.inbrowser.app/pages/common/kubectl-events
