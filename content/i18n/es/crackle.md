# crackle

Crackea y descifra el cifrado Bluetooth Low Energy (BLE).

## Usage
- `crackle -i ruta/a/entrada.pcap` : comprueba si las comunicaciones BLE grabadas contienen los paquetes necesarios para recuperar claves temporales (TKs)
- `crackle -i ruta/a/entrada.pcap -o ruta/a/desencriptado.pcap` : utiliza la fuerza bruta para recuperar la TK de los eventos de emparejamiento registrados y la utiliza para descifrar todas las comunicaciones posteriores
- `crackle -i ruta/a/entrada.pcap -o ruta/a/descifrar.pcap -l 81b06facd90fe7a6e9bbd9cee59736a7` : utiliza la clave a largo plazo (LTK) especificada para descifrar la comunicación grabada

## Links
- More information: https://github.com/mikeryan/crackle
