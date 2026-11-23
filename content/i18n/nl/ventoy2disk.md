# ventoy2disk

Een tool om opstartbare USB-schijven te maken met ISO-bestanden op Windows-systemen.

## Usage
- `Ventoy2Disk VTOYCLI /I /Drive:D:` : installeer Ventoy op schijf D: met standaardinstellingen (MBR, Secure Boot ingeschakeld)
- `Ventoy2Disk VTOYCLI /I /Drive:D: /GPT /NOSB` : installeer Ventoy met GPT-partitiestijl en schakel Secure Boot uit
- `Ventoy2Disk VTOYCLI /I /Drive:D: /R:4096` : installeer Ventoy en reserveer 4GB ruimte aan het einde van de schijf
- `Ventoy2Disk VTOYCLI /I /PhyDrive:1 /FS:NTFS` : installeer Ventoy met fysiek schijfnummer 1 en het NTFS-bestandssysteem
- `Ventoy2Disk VTOYCLI /I /Drive:D: /NOUSBCheck` : installeer Ventoy zonder controle op het USB-type (voor interne schijven)
- `Ventoy2Disk VTOYCLI /U /Drive:D:` : update Ventoy op schijf D: met behoud van de huidige instellingen
- `Ventoy2Disk VTOYCLI /I /Drive:D: /NonDest` : voer een niet-destructieve installatie uit (behoud bestaande gegevens)
- `Ventoy2Disk VTOYCLI /I /Drive:D: /GPT /NOSB /R:2048 /FS:NTFS /NOUSBCheck` : installeer Ventoy met alle opties: GPT, geen Secure Boot, 2GB gereserveerd, NTFS, geen USB-controle

## Links
- More information: https://www.ventoy.net/en/doc_windows_cli.html
