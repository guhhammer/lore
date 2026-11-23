# naabu

Un escáner de puertos rápido escrito en Go que se centra en la fiabilidad y la simplicidad. Nota: Algunas funciones solo se activan cuando `naabu` se ejecuta con privilegios de root, como el escaneo SYN.

## Usage
- `sudo naabu -host host` : ejecuta un escaneo SYN en los puertos predeterminados (los 100 principales) del host remoto
- `naabu -il` : muestra las interfaces de red disponibles y la dirección IP pública del host local
- `naabu -p - -host host` : escanea todos los puertos del host remoto (escaneo CONNECT sin `sudo`)
- `naabu -tp 1000 -host host` : escanea los 1000 puertos principales del host remoto
- `naabu -p 80,443,u:53 -host host` : escanea los puertos TCP 80, 443 y el puerto UDP 53 del host remoto
- `naabu -p 80,443 -cdn -host host` : muestra el tipo de CDN que utiliza el host remoto, si lo hay
- `sudo naabu -v -host host -nmap-cli 'nmap -v -T5 -sC'` : ejecuta `nmap` desde `naabu` para obtener funcionalidades adicionales (`nmap` debe estar instalado)

## Links
- More information: https://docs.projectdiscovery.io/opensource/naabu/running
