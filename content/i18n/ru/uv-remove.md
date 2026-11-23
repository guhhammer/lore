# uv remove

Удалять зависимости из файла `pyproject.toml` проекта.

## Usage
- `uv remove пакет` : удалить зависимость из проекта
- `uv remove пакет1 пакет2 ...` : удалить несколько зависимостей
- `uv remove --dev пакет` : удалить зависимость разработки
- `uv remove --optional имя_дополнения пакет` : удалить зависимость из необязательной группы
- `uv remove --group имя_группы пакет` : удалить зависимость из определённой группы
- `uv remove --no-sync пакет` : удалить без синхронизации виртуального окружения

## Links
- More information: https://docs.astral.sh/uv/reference/cli/#uv-remove
