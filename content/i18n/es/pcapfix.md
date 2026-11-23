# pcapfix

Repara archivos PCAP y PcapNG dañados o corruptos.

## Usage
- `pcapfix ruta/al/archivo.pcapng` : repara un archivo PCAP/PcapNG (Nota: para los archivos PCAP, sólo se escanean los primeros 262144 bytes de cada paquete)
- `pcapfix --deep-scan ruta/al/archivo.pcap` : repara un archivo PCAP completo
- `pcapfix --outfile ruta/al/archivo_reparado.pcap ruta/al/archivo.pcap` : repara un archivo PCAP/PcapNG y escribe el archivo reparado en la ubicación especificada
- `pcapfix --pcapng ruta/al/archivo.pcapng` : repara un archivo PcapNG y lo trata como un archivo PcapNG, ignorando el reconocimiento automático
- `pcapfix --verbose ruta/al/archivo.pcap` : repara un archivo y muestra el proceso en detalle

## Links
- More information: https://f00l.de/pcapfix/
