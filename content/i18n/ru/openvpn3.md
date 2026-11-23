# openvpn3

Клиент OpenVPN 3 для Linux.

## Usage
- `openvpn3 session-start -c путь/к/конфигурации.conf` : запустить новую VPN-сессию
- `openvpn3 sessions-list` : показать список установленных сессий
- `openvpn3 session-manage -c путь/к/конфигурации.conf -D` : отключить текущую сессию, запущенную с указанной конфигурацией
- `openvpn3 config-import -c путь/к/конфигурации.conf` : импортировать конфигурацию VPN
- `openvpn3 configs-list` : показать список импортированных конфигураций

## Links
- More information: https://community.openvpn.net/openvpn/wiki/OpenVPN3Linux
