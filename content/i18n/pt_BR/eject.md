# eject

Ejeta CDs, disquetes, unidades de fita e dispositivos USB.

## Usage
- `eject -d` : exibe o dispositivo padrão
- `eject` : ejeta o dispositivo padrão
- `eject /dev/cdrom` : ejeta um dispositivo específico (a ordem padrão é: cd-rom, scsi, disquete e fita)
- `eject -T /dev/cdrom` : alterna o estado da bandeja de um dispositivo (abre/fecha)
- `eject -r /dev/cdrom` : ejeta uma unidade de CD
- `eject -f /mnt/floppy` : ejeta uma unidade de disquete
- `eject -q /mnt/tape` : ejeta uma unidade de fita
- `eject -i on|off` : define se o botão de ejeção físico é ignorado (`on` impede a ejeção)

## Links
- More information: https://manned.org/eject
