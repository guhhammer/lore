# uv pip

Предоставлять pip-подобные команды для установки, удаления и управления пакетами.

## Usage
- `uv pip install пакет` : установить пакет
- `uv pip install -r requirements.txt` : установить пакеты из файла зависимостей
- `uv pip install пакет==1.2.3` : установить пакет определённой версии
- `uv pip uninstall пакет` : удалить пакет
- `uv pip compile pyproject.toml -o requirements.txt` : зафиксировать зависимости из `pyproject.toml` в `requirements.txt`
- `uv pip list` : вывести список установленных пакетов
- `uv pip show пакет` : показать информацию об установленном пакете
- `uv pip sync requirements.txt` : синхронизировать окружение с файлом зависимостей (установить/удалить для точного соответствия)

## Links
- More information: https://docs.astral.sh/uv/reference/cli/#uv-pip
