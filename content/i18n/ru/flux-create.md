# flux create

Создавать или обновлять ресурсы Flux.

## Usage
- `flux create source git имя_источника --url https://github.com/repository --branch имя_ветки` : создать источник GitRepository
- `flux create kustomization имя_kustomization --source имя_источника --path путь/к/каталогу` : создать ресурс Kustomization для синхронизации каталога из Git-источника
- `flux create helmrelease имя_релиза --chart имя_чарта --source HelmRepository/имя_репозитория` : создать HelmRelease
- `flux create secret git имя_секрета --url https://github.com/repository --username "имя_пользователя" --password "пароль"` : создать секрет для аутентификации в Git-репозитории

## Links
- More information: https://fluxcd.io/flux/cmd/flux_create/
