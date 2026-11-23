# ufw

Простой межсетевой экран (Uncomplicated Firewall). Интерфейс для `iptables`, упрощающий настройку межсетевого экрана. Некоторые подкоманды, такие как `allow`, `enable`, `delete` и др., имеют собственную документацию по использованию.

## Usage
- `sudo ufw enable|disable` : включить/Отключить `ufw`
- `sudo ufw status numbered` : показать правила `ufw` вместе с их номерами
- `sudo ufw allow 5432 comment "Сервис"` : разрешить входящий трафик на порт 5432 на данном хосте с комментарием, идентифицирующим сервис
- `sudo ufw allow proto tcp from 192.168.0.4 to any port 22` : разрешить только TCP-трафик от 192.168.0.4 на любой адрес данного хоста, на порт 22
- `sudo ufw allow in on eth0 from 192.168.0.0/24 to any port 53` : разрешить трафик с любого IP в подсети 192.168.0.0/24 на данном хосте, через порт 53, только на интерфейсе eth0
- `sudo ufw deny 80` : запретить трафик на порт 80 на данном хосте
- `sudo ufw deny proto udp from any to any port 8412:8500` : запретить весь UDP-трафик на порты в диапазоне 8412:8500
- `sudo ufw delete номер_правила` : удалить конкретное правило. Номер правила можно получить из команды `ufw status numbered`

## Links
- More information: https://manned.org/ufw
