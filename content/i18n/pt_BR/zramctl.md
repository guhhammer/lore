# zramctl

Configura e controla dispositivos zram. Use `mkfs` ou `mkswap` para formatar dispositivos zram para partições.

## Usage
- `lsmod | grep -i zram` : verifica se o zram está habilitado
- `sudo modprobe zram` : habilita o zram com um número dinâmico de dispositivos (use `zramctl` para configurar ainda mais os dispositivos)
- `sudo modprobe zram num_devices=2` : habilita o zram com exatamente 2 dispositivos
- `sudo zramctl -f -s 2GB -a lz4` : encontra e inicializa o próximo dispositivo zram gratuito em uma unidade virtual de 2 GB usando a compressão LZ4
- `sudo zramctl` : lista dispositivos atualmente inicializados

## Links
- More information: https://manned.org/zramctl
