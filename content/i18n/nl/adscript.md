# adscript

Compiler voor Adscript-bestanden.

## Usage
- `adscript --output pad/naar/bestand.o pad/naar/invoer_bestand.adscript` : compileer een bestand naar een objectbestand
- `adscript --executable --output pad/naar/bestand pad/naar/invoer_bestand.adscript` : compileer en koppel een bestand aan een zelfstandig uitvoerbaar bestand
- `adscript --llvm-ir --output pad/naar/bestand.ll pad/naar/invoer_bestand.adscript` : compileer een bestand naar LLVM IR in plaats van native machinecode
- `adscript --target-triple i386-linux-elf --output pad/naar/bestand.o pad/naar/invoer_bestand.adscript` : cross-compileer een bestand naar een objectbestand voor een buitenlandse CPU-architectuur of besturingssysteem

## Links
- More information: https://github.com/Amplus2/Adscript
