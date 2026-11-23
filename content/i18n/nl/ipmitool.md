# ipmitool

Interface met de Intelligent Platform Management Interface (IPMI).

## Usage
- `kldload ipmi.ko` : laad de IPMI-kernelmodule voor lokale verbindingen
- `ipmitool shell` : open de IPMI-shell op de lokale hardware
- `ipmitool -H ip_adres -U gebruikersnaam shell` : open de IPMI-shell op een externe host

## Links
- More information: https://man.freebsd.org/cgi/man.cgi?query=ipmitool
