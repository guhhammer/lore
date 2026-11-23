# ip link

Управлять сетевыми интерфейсами.

## Usage
- `ip l` : показать информацию обо всех сетевых интерфейсах
- `ip l sh ethX` : показать информацию о конкретном сетевом интерфейсе
- `sudo ip l s ethX up|down` : включить или выключить сетевой интерфейс
- `sudo ip l s ethX al "LAN Interface"` : присвоить понятное имя сетевому интерфейсу
- `sudo ip l s ethX a ff:ff:ff:ff:ff:ff` : изменить MAC-адрес сетевого интерфейса
- `sudo ip l s ethX mtu 9000` : изменить размер MTU сетевого интерфейса для использования jumbo-кадров
- `sudo ip l s ethX promisc on|off` : включить или выключить неразборчивый режим (promisc) для устройства
- `sudo ip l d ethX` : удалить устройство

## Links
- More information: https://manned.org/ip-link
