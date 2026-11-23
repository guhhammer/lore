# flux bootstrap

Выполнять начальную установку Flux в кластере Kubernetes.

## Usage
- `flux bootstrap github --owner owner --repository repository --path путь/к/каталогу_кластера --personal` : выполнить начальную установку с использованием репозитория GitHub
- `flux bootstrap gitlab --owner owner --repository repository --path путь/к/каталогу_кластера` : выполнить начальную установку с использованием репозитория GitLab
- `flux bootstrap git --url ssh://git@example.com/repository.git --branch main --path путь/к/каталогу_кластера` : выполнить начальную установку из произвольного Git-репозитория через SSH

## Links
- More information: https://fluxcd.io/flux/cmd/flux_bootstrap/
