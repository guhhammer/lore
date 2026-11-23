# nmcli general

Administra los ajustes generales de NetworkManager.

## Usage
- `nmcli g` : muestra el estado general de NetworkManager
- `nmcli g h` : muestra el nombre del host del dispositivo actual
- `sudo nmcli g h nuevo_nombre` : cambia el nombre del host del dispositivo actual
- `nmcli g p` : muestra los permisos de NetworkManager
- `nmcli g l` : muestra el nivel actual de registro (logging) y los dominios
- `sudo nmcli g l l INFO|OFF|ERR|WARN|DEBUG|TRACE domain dominio_1,dominio_2,...` : establece el nivel de registro y/o dominios (mira `man NetworkManager.conf` para todos los dominios disponibles)
