# flux suspend

Приостанавливать ресурсы Flux.

## Usage
- `flux suspend kustomization имя_kustomization` : приостановить согласование для Kustomization
- `flux suspend kustomization --all` : приостановить согласование для всех Kustomization в пространстве имён
- `flux suspend helmrelease имя_релиза` : приостановить согласование для HelmRelease
- `flux suspend source git имя_источника` : приостановить согласование для источника GitRepository
- `flux suspend image update имя_автоматизации` : приостановить согласование для ресурса ImageUpdateAutomation

## Links
- More information: https://fluxcd.io/flux/cmd/flux_suspend/
