# autorecon

Herramienta de reconocimiento de red multihilo que realiza una enumeración automatizada de servicios.

## Usage
- `sudo autorecon host_o_ip1,host_o_ip2,...` : realiza reconocimiento sobre el(los) host(s) objetivo(s) (los resultados del escaneo detallado se guardarán en `./results`)
- `sudo autorecon -t ruta/al/archivo` : realiza reconocimiento sobre el(los) objetivo(s) desde un archivo
- `sudo autorecon -o ruta/a/resultados host_o_ip1,host_o_ip2,...` : guarda los resultados en un directorio diferente
- `sudo autorecon -p T:21-25,80,443,U:53,B:123 host_o_ip1,host_o_ip2,...` : limita el escaneo a puertos y protocolos específicos (`T` para TCP, `U` para UDP, `B` para ambos)

## Links
- More information: https://github.com/AutoRecon/AutoRecon
