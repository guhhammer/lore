# radare2

Een set van reverse engineering tools.

## Usage
- `radare2 -nw pad/naar/binary` : open een bestand in schrijfmodus zonder het parsen van de bestandsformaatheaders
- `radare2 -d pad/naar/binary` : debug een programma
- `radare2 -i pad/naar/script.r2 pad/naar/binary` : voer een script uit voordat de interactieve CLI start
- `radare2_commando?` : [Interactief] Toon de helptekst voor ieder commando
- `!shell_commando` : [Interactief] Voer een shellcommando uit vanuit de interactieve CLI
- `pr >pad/naar/bestand.bin` : [Interactief] Dump raw bytes van het huidige block naar een bestand

## Links
- More information: https://book.rada.re/first_steps/commandline_flags.html
