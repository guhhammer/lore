# adscript

Compilatore per file Adscript.

## Usage
- `adscript --output percorso/del/file.o percorso/del/file_input.adscript` : compila un file in un file oggetto
- `adscript --executable --output percorso/del/file percorso/del/file_input.adscript` : compila e linka un file in un eseguibile standalone
- `adscript --llvm-ir --output percorso/del/file.ll percorso/del/file_input.adscript` : compila un file in LLVM IR invece che in codice macchina nativo
- `adscript --target-triple i386-linux-elf --output percorso/del/file.o percorso/del/file_input.adscript` : cross-compila un file in un file oggetto per un'architettura CPU o sistema operativo straniero

## Links
- More information: https://github.com/Amplus2/Adscript
