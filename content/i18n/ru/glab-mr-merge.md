# glab mr merge

Сливать запросы на слияние GitLab.

## Usage
- `glab mr merge` : слить запрос на слияние, связанный с текущей веткой, в интерактивном режиме
- `glab mr merge номер_запроса` : слить указанный запрос на слияние в интерактивном режиме
- `glab mr merge -d` : слить запрос на слияние, удалив ветку локально и в удалённом репозитории
- `glab mr merge -s -m "тело_сообщения_коммита"` : сжать текущий запрос на слияние в один коммит с указанным сообщением и слить
- `glab mr merge --help` : показать справку

## Links
- More information: https://gitlab.com/gitlab-org/cli/-/blob/main/docs/source/mr/merge.md
