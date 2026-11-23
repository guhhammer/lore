# resolvectl

Разрешать доменные имена, IPv4- и IPv6-адреса, DNS-записи ресурсов и сервисы. Просматривать и перенастраивать DNS-резолвер.

## Usage
- `resolvectl` : показать настройки DNS
- `resolvectl query домен1 домен2 ...` : разрешить IPv4- и IPv6-адреса для одного или нескольких доменов
- `resolvectl query ip_адрес` : получить домен указанного IP-адреса
- `resolvectl flush-caches` : очистить все локальные кеши DNS
- `resolvectl statistics` : показать статистику DNS (транзакции, кеш и вердикты DNSSEC)
- `resolvectl --legend no -t MX query домен` : получить MX-запись домена
- `resolvectl service _сервис._протокол имя` : разрешить SRV-запись, например _xmpp-server._tcp gmail.com
- `resolvectl tlsa tcp домен:443` : получить TLS-ключ

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/resolvectl.html
