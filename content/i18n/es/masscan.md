# masscan

Un escáner de red muy rápido. Funciona mejor con privilegios elevados. Para obtener ayuda sobre la compatibilidad con Nmap, ejecute `masscan --nmap`.

## Usage
- `masscan ip_address|network_prefix -p 80` : escanea una IP o subred de red en busca del puerto 80
- `masscan 10.0.0.0/16 --top-ports 100 --rate 100000` : escanea una subred de clase B en busca de los 100 puertos principales a 100,000 paquetes por segundo
- `masscan 10.0.0.0/16 --top-ports 100 --excludefile ruta/al/archivo` : escanea una subred de clase B evitando rangos de un archivo de exclusión específico
- `masscan 10.0.0.0/16 -p 22,80 --banners --rate 100000` : escanea una subred de clase B con detección de versión similar a Nmap (captura de banner)
- `masscan 0.0.0.0/0 -p 80,443 --rate 10000000` : escanea internet en busca de servidores web que se ejecutan en los puertos 80 y 443
- `masscan 0.0.0.0/0 -p U:53 --rate 10000000` : escanea internet en busca de servidores DNS que se ejecutan en el puerto UDP 53
- `masscan 0.0.0.0/0 -p 0-65535 --output-format binary|grepable|json|list|xml --output-filename ruta/al/archivo` : escanea internet en busca de un rango de puertos específico y exporta los resultados a un archivo
- `masscan --readscan ruta/al/archivo` : lee los resultados del escaneo binario de un archivo y los envía a `stdout`

## Links
- More information: https://manned.org/masscan
