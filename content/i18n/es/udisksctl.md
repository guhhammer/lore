# udisksctl

Interactúa con `udisksd` para consultar y manipular dispositivos de almacenamiento.

## Usage
- `udisksctl status` : muestra información general sobre unidades de disco y dispositivos de bloque
- `udisksctl info -b /dev/sdX` : muestra información detallada sobre un dispositivo
- `udisksctl info -b /dev/sdXN` : muestra información detallada sobre una partición de un dispositivo
- `udisksctl mount -b /dev/sdXN` : monta una partición de un dispositivo y muestra el punto de montaje
- `udisksctl unmount -b /dev/sdXN` : desmonta una partición de un dispositivo
- `udisksctl power-off -b /dev/sdX` : apaga un dispositivo para extraerlo de forma segura
- `udisksctl monitor` : supervisa el programa residente en busca de eventos

## Links
- More information: https://storaged.org/doc/udisks2-api/latest/udisksctl.1.html
