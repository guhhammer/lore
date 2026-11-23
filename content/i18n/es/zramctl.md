# zramctl

Configura y controla dispositivos zram. Usa `mkfs` o `mkswap` para formatear dispositivos zram a particiones.

## Usage
- `lsmod | grep -i zram` : comprueba si zram está habilitado
- `sudo modprobe zram` : habilita zram con un número dinámico de dispositivos (usa `zramctl` para configurar más dispositivos)
- `sudo modprobe zram num_devices=2` : habilita zram con exactamente 2 dispositivos
- `sudo zramctl -f -s 2GB -a lz4` : encuentra e inicializa el siguiente dispositivo zram libre a una unidad virtual de 2 GB usando compresión LZ4
- `sudo zramctl` : lista los dispositivos actualmente inicializados

## Links
- More information: https://manned.org/zramctl
