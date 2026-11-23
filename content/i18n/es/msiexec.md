# msiexec

Instala, actualiza, repara o desinstala programas de Windows utilizando archivos de paquetes MSI y MSP.

## Usage
- `msiexec /package ruta\al\archivo.msi` : instala un programa desde su paquete MSI
- `msiexec /package https://example.com/instalador.msi` : instala un paquete MSI desde un sitio web
- `msiexec /update ruta\al\archivo.msp` : instala un archivo de parche MSP
- `msiexec /uninstall ruta\al\archivo` : desinstala un programa o parche utilizando su respectivo archivo MSI o MSP
