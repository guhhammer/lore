# nmcli device

Gestiona interfaces de red y establece nuevas conexiones WiFi usando NetworkManager.

## Usage
- `nmcli d` : muestra los estados de todas las interfaces de red
- `nmcli d w` : muestra los puntos de acceso WiFi disponibles
- `nmcli d w c ssid -a` : se conecta a una red WiFi con el SSID especificado (se te pedirá una contraseña)
- `nmcli d w s` : muestra la contraseña y el código QR para la red WiFi actual
- `nmcli d sh wlan0` : imprime información detallada sobre un dispositivo
