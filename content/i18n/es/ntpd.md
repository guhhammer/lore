# ntpd

El daemon oficial de NTP (Network Time Protocol) para sincronizar el reloj del sistema con servidores de tiempo remotos o relojes de referencia locales.

## Usage
- `sudo ntpd` : inicia el daemon
- `sudo ntpd --quit` : sincroniza la hora del sistema con servidores remotos una sola vez (sale después de sincronizar)
- `sudo ntpd --panicgate --quit` : sincroniza una sola vez permitiendo "grandes" ajustes

## Links
- More information: https://manned.org/ntpd
