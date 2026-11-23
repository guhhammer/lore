# aircrack-ng

Crackea claves WEP y WPA/WPA2 a partir de handshake en paquetes capturados. Parte de la suite de software de red Aircrack-ng.

## Usage
- `aircrack-ng -w ruta/a/listado.txt ruta/a/captura.cap` : crackea la clave desde el archivo de captura usando un listado
- `aircrack-ng -p número -w ruta/a/listado.txt ruta/a/captura.cap` : crackea la clave usando múltiples hilos de CPU desde el archivo de captura usando una lista
- `aircrack-ng -w ruta/a/listado.txt -e extended_ssid ruta/a/captura.cap` : descifra la clave del archivo de captura utilizando un listado y el essid del punto de acceso
- `aircrack-ng -w ruta/a/listado.txt --bssid mac ruta/a/captura.cap` : descifra la clave del archivo de captura utilizando una listado y la dirección MAC del punto de acceso

## Links
- More information: https://www.aircrack-ng.org/doku.php?id=aircrack-ng
