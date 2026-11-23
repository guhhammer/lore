# nmap

Herramienta de exploración de redes y escáner de seguridad/puertos. Algunas funciones (por ejemplo, el escaneo SYN) solo se activan cuando se ejecuta `nmap` con privilegios de root.

## Usage
- `nmap -v1|2|3 ip_o_nombre_del_host` : escanea los 1000 puertos principales de un host remoto con varios niveles de verbosidad
- `nmap -T5 -sn 192.168.0.0/24|ip_o_nombre_del_host1,ip_o_nombre_del_host2,...` : ejecuta un barrido de ping sobre toda una subnet o hosts individuales de forma muy agresiva
- `sudo nmap -A -iL ruta/al/archivo.txt` : habilita la detección del sistema operativo, la detección de la versión, el escaneo de scripts y el traceroute de los hosts desde un archivo
- `nmap -p puerto1,puerto2,... ip_o_host1,ip_o_host2,...` : escanea una lista específica de puertos (usa `-p-` para todos los puertos del 1 al 65535)
- `nmap -sC -sV -oA top-1000-ports ip_o_host1,ip_o_host2,...` : realiza la detección de servicios y versiones de los 1000 puertos principales utilizando scripts NSE predeterminados, escribiendo los resultados (`-oA`) en archivos de salida
- `nmap --script "default and safe" ip_o_host1,ip_o_host2,...` : escanea los objetivos cuidadosamente utilizando scripts NSE "predeterminados y seguros"
- `nmap --script "http-*" ip_o_host1,ip_o_host2,... -p 80,443` : escanea los servidores web que se ejecutan en los puertos estándar 80 y 443 utilizando todos los scripts NSE "http-*" disponibles
- `sudo nmap -T0 -D señuelo_ip1,señuelo_ip2,... --source-port 53 -f --data-length 16 -Pn ip_o_host` : intenta evadir la detección de IDS/IPS utilizando un escaneo extremadamente lento (`-T0`), direcciones de origen De señuelo, paquetes fragmentados, datos aleatorios y otros métodos

## Links
- More information: https://nmap.org/book/man.html
