# dnf download

Загружать RPM-пакеты из репозиториев DNF. Не является частью основной команды `dnf`, но поддерживается через плагин `dnf-plugins-core`.

## Usage
- `dnf download пакет` : скачать последнюю версию пакета в текущий каталог
- `dnf download пакет --destdir путь/к/каталогу` : скачать пакет в указанный каталог (каталог должен существовать)
- `dnf download --url пакет` : вывести URL, откуда можно скачать RPM-пакет

## Links
- More information: https://dnf-plugins-core.readthedocs.io/en/latest/download.html
