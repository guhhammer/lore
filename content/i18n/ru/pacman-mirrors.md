# pacman-mirrors

Генерировать список зеркал `pacman` для Manjaro Linux. Каждый запуск `pacman-mirrors` требует синхронизации базы данных и обновления системы с помощью `sudo pacman -Syyu`.

## Usage
- `sudo pacman-mirrors --fasttrack` : сгенерировать список зеркал с настройками по умолчанию
- `pacman-mirrors --status` : получить статус текущих зеркал
- `pacman-mirrors --get-branch` : показать текущую ветку
- `sudo pacman-mirrors --api --set-branch stable|unstable|testing` : переключиться на другую ветку
- `pacman-mirrors --country-list` : вывести список доступных стран
- `sudo pacman-mirrors --country United_States,France,Germany,...` : сгенерировать список зеркал, используя только зеркала в конкретных странах

## Links
- More information: https://wiki.manjaro.org/index.php?title=Pacman-mirrors
