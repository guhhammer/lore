# sysctl

Laufzeit-Kernelparameter auflisten und ändern.

## Usage
- `sysctl -a` : liste alle verfügbaren Kernelparameter mit ihren Werten auf
- `sysctl -w sektion.tunable=wert` : setze einen veränderbaren Kernelparameter
- `sysctl fs.file-nr` : frage aktuell geöffnete Datei-Handler ab
- `sysctl fs.file-max` : frage die maximale Anzahl geöffneter Dateien ab
- `sysctl -p` : übernimm Änderungen aus der `/etc/sysctl.conf` Datei

## Links
- More information: https://manned.org/sysctl.8
