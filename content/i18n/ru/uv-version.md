# uv version

Читать или обновлять версию проекта.

## Usage
- `uv version` : показать текущую версию проекта
- `uv version 1.2.3` : установить определённую версию проекта
- `uv version --bump major|minor|patch` : увеличить версию проекта по семантическому версионированию
- `uv version --bump patch --dry-run` : предварительно просмотреть изменение версии без записи в `pyproject.toml`
- `uv version --package имя_пакета 1.2.3` : обновить версию определённого пакета в рабочем пространстве
- `uv version --output-format json` : показать версию в формате JSON

## Links
- More information: https://docs.astral.sh/uv/reference/cli/#uv-version
