# uv cache

Управлять глобальным каталогом кеша `uv`.

## Usage
- `uv cache dir` : показать путь к каталогу кеша
- `uv cache clean` : очистить весь кеш (удалить все кешированные пакеты и окружения)
- `uv cache clean пакет1 пакет2 ...` : очистить кеш для определённых пакетов
- `uv cache prune` : удалить все недостижимые объекты из кеша
- `uv cache prune --ci` : очистить кеш с оптимизацией для CI-окружений (например, GitHub Actions)
- `uv cache clean --cache-dir путь/к/кешу` : указать определённый каталог кеша
- `uv cache clean -v` : очистить кеш с подробным выводом

## Links
- More information: https://docs.astral.sh/uv/reference/cli/#uv-cache
