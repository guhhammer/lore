# pihole

Управлять DNS-сервером для блокировки рекламы Pi-hole.

## Usage
- `pihole status` : проверить статус Pi-hole
- `sudo pihole -up` : обновить Pi-hole и Gravity
- `pihole enable|disable` : запустить или остановить демона
- `pihole reloaddns` : обновить списки и очистить кэш без перезапуска DNS-сервера
- `pihole -g` : обновить список доменов, раздающих рекламу
- `pihole allow|deny example.com` : разрешить или запретить указанный домен
- `pihole -q example.com` : искать домен в списках
- `pihole -t` : открыть лог подключений в реальном времени

## Links
- More information: https://docs.pi-hole.net/main/pihole-command/
