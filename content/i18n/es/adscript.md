# adscript

Compilador de archivos Adscript.

## Usage
- `adscript --output ruta/al/archivo.o ruta/al/archivo_de_entrada.adscript` : compila un archivo en un archivo objeto
- `adscript --executable --output ruta/al/archivo ruta/al/archivo_entrada.adscript` : compila y vincula un archivo a un ejecutable independiente
- `adscript --llvm-ir --output ruta/al/archivo.ll ruta/al/archivo_entrada.adscript` : compila un archivo a LLVM IR en lugar de código de máquina nativo
- `adscript --target-triple i386-linux-elf --output ruta/al/archivo.o ruta/al/archivo_entrada.adscript` : crea un archivo con código objeto para otra arquitectura u otro sistema operativo

## Links
- More information: https://github.com/Amplus2/Adscript
