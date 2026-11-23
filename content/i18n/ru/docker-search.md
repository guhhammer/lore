# docker search

Искать образы Docker на Docker Hub.

## Usage
- `docker search ключевое_слово` : найти образы Docker по имени или ключевому слову
- `docker search -f is-official=true ключевое_слово` : найти образы и показать только официальные
- `docker search -f is-automated=true ключевое_слово` : найти образы и показать только автоматические сборки
- `docker search -f stars=число ключевое_слово` : найти образы с минимальным количеством звёзд
- `docker search --limit число ключевое_слово` : ограничить количество результатов
- `docker search -f "\{\{.Name\}\}: \{\{.Description\}\}" ключевое_слово` : настроить формат вывода

## Links
- More information: https://docs.docker.com/reference/cli/docker/search/
