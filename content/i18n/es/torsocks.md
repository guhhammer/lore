# torsocks

Enruta el tráfico de cualquier aplicación a través de la red Tor.

## Usage
- `torsocks comando` : ejecuta un comando usando Tor
- `. torsocks on|off` : activa o desactiva Tor en este intérprete de comandos
- `torsocks --shell` : lanza una nueva interfaz de comando habilitada con Tor
- `torsocks show` : revisa si la interfaz de comando actual está habilitada con Tor (`$LD_PRELOAD` este valor estará vacío si no está habilitada)
- `torsocks -i curl https://check.torproject.org/api/ip` : aísla el tráfico a través de un circuito Tor diferente, mejorando el anonimato
- `torsocks -a ip_address -P puerto comando` : se conecta a un proxy Tor que se ejecuta en una dirección y un puerto específico

## Links
- More information: https://manned.org/torsocks
