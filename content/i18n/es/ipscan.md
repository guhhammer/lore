# ipscan

Un rápido escáner de red diseñado para ser simple de usar. También conocido como Angry IP Scanner.

## Usage
- `ipscan 192.168.0.1` : escanea una dirección IP específica
- `ipscan 192.168.0.1-254` : escanea un rango de direcciones IP
- `ipscan 192.168.0.1-254 -o ruta/a/salida.txt` : escanea un rango de direcciones IP y guarda los resultados en un archivo
- `ipscan 192.168.0.1-254 -p 80,443,22` : escanea IPs con un conjunto específico de puertos
- `ipscan 192.168.0.1-254 -d 200` : escanea con un retardo entre peticiones para evitar la congestión de la red
- `ipscan --help` : muestra la ayuda

## Links
- More information: https://www.aldeid.com/wiki/Angry-IPScan#CLI
