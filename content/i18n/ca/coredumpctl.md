# coredumpctl

Recupera i processa volcats de memòria i les seves metadades.

## Usage
- `coredumpctl list` : llista tots els volcats de memòria capturats
- `coredumpctl list programa` : llista tots els volcats de memòria capturats per un programa
- `coredumpctl info pid` : mostra informació sobre els volcats de memòria que coincideixin amb el `PID` d'un programa
- `coredumpctl debug programa` : invoca el depurador fent servir l'últim volcat de memòria per un programa
- `coredumpctl --output ruta/al/arxiu dump programa` : extreu l'últim volcat de memòria a un fitxer

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/coredumpctl.html
