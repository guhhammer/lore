# udisksctl

Communiceer met `udisksd` om opslagapparaten te bevragen en te manipuleren.

## Usage
- `udisksctl status` : toon algemene informatie over schijfstations en blokapparaten
- `udisksctl info -b /dev/sdX` : toon gedetailleerde informatie over een apparaat
- `udisksctl info -b /dev/sdXN` : toon gedetailleerde informatie over een apparaatpartitie
- `udisksctl mount -b /dev/sdXN` : koppel een apparaatpartitie aan en toon het koppelpunt
- `udisksctl unmount -b /dev/sdXN` : ontkoppel een apparaatpartitie
- `udisksctl power-off -b /dev/sdX` : zet een apparaat uit om het veilig te verwijderen
- `udisksctl monitor` : monitor de daemon op gebeurtenissen

## Links
- More information: https://storaged.org/doc/udisks2-api/latest/udisksctl.1.html
