# uv export

Экспортировать файл блокировки проекта в альтернативный формат.

## Usage
- `uv export --format requirements-txt -o requirements.txt` : экспортировать зависимости в файл `requirements.txt`
- `uv export --format pylock.toml` : экспортировать зависимости в формат `pylock.toml`
- `uv export --no-dev` : экспортировать только рабочие зависимости (без зависимостей разработки)
- `uv export --extra имя_группы` : экспортировать с определённой необязательной группой зависимостей
- `uv export --all-extras` : экспортировать со всеми необязательными зависимостями
- `uv export --group имя_группы` : экспортировать с определённой группой зависимостей
- `uv export --no-hashes` : экспортировать без хешей
- `uv export --package имя_пакета` : экспортировать зависимости для определённого пакета в рабочем пространстве

## Links
- More information: https://docs.astral.sh/uv/reference/cli/#uv-export
