# traceroute

Выводить маршрут следования пакетов до сетевого хоста.

## Usage
- `traceroute example.com` : выполнить трассировку до хоста
- `traceroute -n example.com` : отключить преобразование IP-адресов в имена хостов
- `traceroute -w 0.5 example.com` : указать время ожидания ответа в секундах
- `traceroute -q 5 example.com` : указать количество запросов на каждый узел (hop)
- `traceroute example.com 42` : указать размер отправляемого пакета в байтах
- `traceroute --mtu example.com` : определить MTU до узла назначения
- `traceroute -I example.com` : использовать протокол ICMP вместо UDP для трассировки

## Links
- More information: https://manned.org/traceroute
