# ip neighbor

Подкоманда IP для управления таблицами соседей (ARP/NDP).

## Usage
- `ip n` : показать записи таблицы соседей (ARP)
- `sudo ip n f dev ethX` : удалить записи из таблицы соседей на устройстве `ethX`
- `ip n g ip_адрес_поиска dev ethX` : выполнить поиск соседа и вернуть запись о нём
- `sudo ip n add|delete ip_адрес lladdr mac_адрес dev ethX nud reachable` : добавить или удалить запись ARP для IP-адреса соседа на `ethX`
- `sudo ip n change|replace ip_адрес lladdr новый_mac_адрес dev ethX` : изменить или заменить запись ARP для IP-адреса соседа на `ethX`
- `ip -6|-4 n` : показать только соседей IPv6 или IPv4

## Links
- More information: https://manned.org/ip-neighbour
