# flux

Решение для непрерывной и прогрессивной доставки в Kubernetes.

## Usage
- `flux check --pre` : проверить, выполнены ли предварительные требования Flux в кластере
- `flux bootstrap github|gitlab --owner owner --repository repository --path путь/к/каталогу_кластера` : выполнить начальную установку Flux в кластере Kubernetes
- `flux create source git имя_источника --url https://github.com/repository --branch имя_ветки` : создать новый источник GitRepository
- `flux get all` : показать все пользовательские ресурсы Flux
- `flux reconcile kustomization имя --with-source` : запустить согласование для конкретной Kustomization и применить изменения из источника
- `flux suspend kustomization имя` : приостановить согласование для Kustomization
- `flux resume kustomization имя` : возобновить согласование для Kustomization
- `flux подкоманда --help` : показать справку по конкретной подкоманде

## Links
- More information: https://fluxcd.io/flux/cmd/
