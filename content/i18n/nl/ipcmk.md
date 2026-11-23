# ipcmk

Maak IPC-resources (Inter-process Communication) aan.

## Usage
- `ipcmk -M segment_size_in_bytes` : maak een gedeeld geheugensegment aan
- `ipcmk -S element_size` : maak een semafoor aan
- `ipcmk -Q` : maak een berichtenwachtrij aan
- `ipcmk -M segment_size_in_bytes octal_permissions` : maak een gedeeld geheugensegment aan met specifieke permissies (standaard is 0644)

## Links
- More information: https://manned.org/ipcmk
