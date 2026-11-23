# ip address

Подкоманда для управления IP-адресами.

## Usage
- `ip a` : показать список сетевых интерфейсов и связанных с ними IP-адресов
- `ip a s up` : показать только активные сетевые интерфейсы
- `ip a s ethX` : показать информацию о конкретном сетевом интерфейсе
- `sudo ip a a ip_адрес dev ethX` : добавить IP-адрес сетевому интерфейсу
- `sudo ip a d ip_адрес dev ethX` : удалить IP-адрес с сетевого интерфейса
- `sudo ip a f ethX scope global|host|link` : удалить все IP-адреса из указанной области действия (scope) для сетевого интерфейса

## Links
- More information: https://manned.org/ip-address
