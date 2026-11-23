# netsh interface portproxy

Configura y muestra el estado de varios componentes de red.

## Usage
- `netsh interface portproxy show all` : muestra la configuración actual de reenvío de puertos
- `netsh interface portproxy add v4tov4 listenaddress=192.168.0.1 listenport=8080 connectaddress=10.0.0.1 connectport=80` : configura el reenvío de puertos IPv4 (ejecutar en consola elevada)
- `netsh interface portproxy delete v4tov4 listenaddress=192.168.0.1 listenport=8080` : elimina el reenvío de puertos IPv4 (ejecutar en consola elevada)
- `netsh interface portproxy` : muestra la ayuda
