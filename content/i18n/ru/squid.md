# squid

Кэшировать и перенаправлять HTTP-запросы через прокси-сервер.

## Usage
- `sudo squid` : запустить Squid в фоновом режиме
- `sudo squid -N` : запустить Squid на переднем плане (не в фоновом режиме)
- `sudo squid -f путь/к/squid.conf` : запустить Squid с указанным файлом конфигурации
- `sudo squid -k parse` : проверить файл конфигурации на наличие ошибок
- `sudo squid -k reconfigure` : перезагрузить файл конфигурации
- `sudo squid -k shutdown` : корректно остановить Squid
- `sudo squid -k rotate` : выполнить ротацию файлов журнала

## Links
- More information: https://manned.org/squid
