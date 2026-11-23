# glab mr create

Управлять запросами на слияние GitLab.

## Usage
- `glab mr create` : создать запрос на слияние в интерактивном режиме
- `glab mr create -f` : создать запрос на слияние, определив заголовок и описание из сообщений коммитов текущей ветки
- `glab mr create --draft` : создать черновой запрос на слияние
- `glab mr create -b целевая_ветка -t "заголовок" -d "описание"` : создать запрос на слияние с указанием целевой ветки, заголовка и описания
- `glab mr create -w` : начать создание запроса на слияние в браузере по умолчанию

## Links
- More information: https://gitlab.com/gitlab-org/cli/-/blob/main/docs/source/mr/create.md
