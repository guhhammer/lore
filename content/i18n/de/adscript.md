# adscript

Compiler für Adscript Dateien.

## Usage
- `adscript --output pfad/zu/datei.o pfad/zu/quelldatei.adscript` : kompiliere eine Datei zu einer Objektdatei
- `adscript --executable --output pfad/zu/datei pfad/zu/quelldatei.adscript` : kompiliere eine Datei zu einer ausführbaren Binärdatei
- `adscript --llvm-ir --output pfad/zu/datei.ll pfad/zu/quelldatei.adscript` : kompiliere eine Datei zu LLVM IR anstelle von nativem Maschinencode
- `adscript --target-triple i386-linux-elf --output pfad/zu/datei.o pfad/zu/quelldatei.adscript` : cross-kompiliere eine Datei zu einer Objektdatei für eine fremde CPU Architektur oder ein fremdes Betriebssystem

## Links
- More information: https://github.com/Amplus2/Adscript
