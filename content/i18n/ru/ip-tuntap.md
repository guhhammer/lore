# ip tuntap

Управлять виртуальными сетевыми интерфейсами TUN/TAP.

## Usage
- `ip tunt` : показать все существующие устройства TUN/TAP
- `sudo ip tunt a d tun0 m t` : создать устройство TUN с конкретным именем
- `sudo ip tunt a d tap0 m ta` : создать устройство TAP с конкретным именем
- `sudo ip tunt d d tun0|tap0 m tun|tap` : удалить устройство TUN или TAP
- `sudo ip tunt a d tun0|tap0 m tun|tap u имя_пользователя` : установить владельца (UID) устройства TUN/TAP
- `sudo ip tunt a d tun0|tap0 m tun|tap u имя_пользователя g имя_группы` : установить владельца (UID) и группу (GID) устройства TUN/TAP

## Links
- More information: https://baturin.org/docs/iproute2/#ip-tuntap
