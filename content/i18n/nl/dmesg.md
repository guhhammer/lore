# dmesg

Schrijf de kernelberichten naar `stdout`.

## Usage
- `sudo dmesg` : toon kernelberichten
- `sudo dmesg -H` : toon kernelberichten in een leesbaar formaat (gelijk aan `dmesg --color --reltime` doorgestuurd naar een pager)
- `sudo dmesg -l err` : toon kernelfoutmeldingen
- `sudo dmesg -w` : toon kernelberichten en blijf wachten op nieuwe berichten, vergelijkbaar met `tail --follow`
- `sudo dmesg --since "1 hour ago"` : toon kernelberichten die in het afgelopen uur zijn verzonden
- `sudo dmesg -e` : toon kernelberichten met tijdstempels als verschillen ten opzichte van de lokale tijd
- `sudo dmesg -T` : toon kernelberichten met een tijdstempel voor elk bericht
- `sudo dmesg -L` : kleur output

## Links
- More information: https://manned.org/dmesg
