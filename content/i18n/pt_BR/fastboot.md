# fastboot

Se comunica com dispositivos Android conectados quando iniciados no modo fastboot (o único lugar em que `adb` não funciona).

## Usage
- `fastboot oem unlock` : desbloqueia o bootloader
- `fastboot oem lock` : bloqueia o bootloader novamente
- `fastboot reboot bootloader` : reinicia o dispositivo no modo fastboot para o modo fastboot novamente
- `fastboot flash arquivo.img` : flasheia uma imagem
- `fastboot flash recovery arquivo.img` : flasheia uma imagem de recovery customizada
- `fastboot devices` : exibe os dispositivos conectados
- `fastboot getvar all` : mostra todas as informações de um dispositivo
