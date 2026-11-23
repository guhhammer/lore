# uv tool

Устанавливать и запускать команды, предоставляемые пакетами Python.

## Usage
- `uv tool run команда` : запустить команду из пакета без его установки
- `uv tool install пакет` : установить пакет Python глобально
- `uv tool install "git+url_репозитория@v1.2.3"` : установить из конкретного git-репозитория по конкретному тегу
- `uv tool upgrade пакет` : обновить установленный пакет Python
- `uv tool uninstall пакет` : удалить пакет Python
- `uv tool list` : вывести список глобально установленных пакетов Python

## Links
- More information: https://docs.astral.sh/uv/reference/cli/#uv-tool
