# zmap

Escáner de red rápido y de código abierto para sondeos en toda Internet.

## Usage
- `sudo zmap -p puerto SUBNETS` : escanea una subred o un espacio IPv4 completo para un puerto TCP específico (por defecto: 80)
- `sudo zmap -p puerto1,puerto2-puerto3,... SUBNETS` : escanea puertos específicos o rangos de puertos en una subred
- `sudo zmap -o ruta/al/archivo_salida.csv -f "saddr,daddr,sport,dport" SUBNETS` : envía los resultados a un archivo CSV con campos personalizados
- `sudo zmap -r paquetes_por_segundo SUBNETS` : limita la velocidad de escaneo a un número específico de paquetes por segundo
- `zmap -d SUBNETS` : realiza un simulacro sin enviar paquetes
- `sudo zmap -b ruta/a/blocklist.txt SUBNETS` : excluye subredes utilizando un archivo de lista de bloqueo en notación CIDR
- `sudo zmap -S ip_origen SUBNETS` : establece una IP de orígen específica para los paquetes de exploración
- `sudo zmap -p puerto1,puerto2-puerto3 -n 1000 SUBNETS` : limita el número/porcentaje de objetivos a sondear (por ejemplo, 1000 pares IP/puerto)

## Links
- More information: https://manned.org/zmap
