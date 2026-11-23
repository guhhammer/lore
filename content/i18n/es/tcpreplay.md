# tcpreplay

Reproduce el tráfico de red almacenado en un archivo `pcap`.

## Usage
- `tcpreplay --listnics` : lista las interfaces de red disponibles
- `tcpreplay -i eth0 tráfico.pcap` : reproduce el tráfico de la interface
- `tcpreplay -i eth0 -v tráfico.pcap` : reproduce tráfico en la interface y en `stdout`
- `tcpreplay -i eth0 -t tráfico.pcap` : reproduce el tráfico en la interface lo más rápido posible
- `tcpreplay -i eth0 -M 10 tráfico.pcap` : reproduce el tráfico en la interface a los Mbps indicados
- `tcpreplay -i eth0 -l número_veces tráfico.pcap` : reproduce el tráfico en la interface varias veces

## Links
- More information: https://tcpreplay.appneta.com/wiki/tcpreplay-man.html
