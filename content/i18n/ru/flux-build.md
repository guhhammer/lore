# flux build

Собирать манифесты Flux.

## Usage
- `flux build kustomization имя --path путь/к/kustomization` : собрать Kustomization
- `flux build kustomization имя --path путь/к/kustomization >путь/к/manifests.yaml` : собрать Kustomization и сохранить манифесты в файл
- `flux build kustomization имя --path путь/к/kustomization | kubectl apply -f -` : собрать Kustomization и передать результат напрямую в `kubectl`

## Links
- More information: https://fluxcd.io/flux/cmd/flux_build/
