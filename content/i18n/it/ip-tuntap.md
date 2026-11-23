# ip tuntap

Gestisce le interfacce di rete virtuali TUN/TAP.

## Usage
- `ip tunt` : mostra tutti i dispositivi TUN/TAP esistenti
- `sudo ip tunt a d tun0 m t` : crea un dispositivo TUN con un nome specifico
- `sudo ip tunt a d tap0 m ta` : crea un dispositivo TAP con un nome specifico
- `sudo ip tunt d d tun0|tap0 m tun|tap` : elimina un dispositivo TUN o TAP
- `sudo ip tunt a d tun0|tap0 m tun|tap u nome_utente` : imposta il proprietario (UID) di un dispositivo TUN/TAP
- `sudo ip tunt a d tun0|tap0 m tun|tap u nome_utente g nome_gruppo` : imposta sia il proprietario (UID) che il gruppo (GID) per un dispositivo TUN/TAP

## Links
- More information: https://baturin.org/docs/iproute2/#ip-tuntap
