# airdecap-ng

Descifra un archivo de captura cifrado (WEP, WPA o WPA2). Parte del paquete de software de red de Aircrack-ng.

## Usage
- `airdecap-ng -b ap_mac ruta/al/archivo_de_captura.cap` : elimina encabezados inalámbricos de una red abierta de archivos de captura y utiliza los puntos de acceso con dirección MAC para filtrar
- `airdecap-ng -w clave_hex ruta/al/archivo_de_captura.cap` : descifra un archivo de captura wEP cifrado usando la clave en formato hexadecimal
- `airdecap-ng -e extended_ssid -p contraseña ruta/al/archivo_de_captura.cap` : descifra un archivo de captura WPA/WPA2 cifrado usando los puntos de acceso essid y contraseña
- `airdecap-ng -l -e extended_ssid -p contraseña ruta/al/archivo_de_captura.cap` : descifra un archivo de captura WPA/WPA2 cifrado preservando los encabezados usando los puntos de acceso essid y contraseña
- `airdecap-ng -b ap_mac -e extended_ssid -p contraseña ruta/al/archivo_de_captura.cap` : descifra un archivo de captura WPA/WPA2 cifrado usando los puntos de acceso essid y contraseña, así como su dirección MAC para filtrar

## Links
- More information: https://www.aircrack-ng.org/doku.php?id=airdecap-ng
