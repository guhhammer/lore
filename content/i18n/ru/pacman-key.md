# pacman-key

Скрипт-обёртка для GnuPG, используемый для управления связкой ключей pacman.

## Usage
- `sudo pacman-key --init` : инициализировать связку ключей `pacman`
- `sudo pacman-key --populate` : добавить ключи Arch Linux по умолчанию
- `pacman-key -l` : вывести ключи из публичной связки ключей
- `sudo pacman-key -a путь/к/файлу_ключей.gpg` : добавить указанные ключи
- `sudo pacman-key -r "uid|имя|email"` : получить ключ с сервера ключей
- `pacman-key -f "uid|имя|email"` : вывести отпечаток конкретного ключа
- `sudo pacman-key --lsign-key "uid|имя|email"` : подписать импортированный ключ локально
- `sudo pacman-key -d "uid|имя|email"` : удалить конкретный ключ

## Links
- More information: https://manned.org/pacman-key
