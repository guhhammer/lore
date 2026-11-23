# bcdboot

Configura o repara archivos de arranque.

## Usage
- `bcdboot C:\Windows` : inicializa la partición del sistema utilizando archivos BCD de la carpeta de Windows de origen
- `bcdboot C:\Windows /v` : habilita el modo verboso
- `bcdboot C:\Windows /s S:` : especifica la letra de volumen de la partición del sistema
- `bcdboot C:\Windows /l es-es` : especifica un idioma local
- `bcdboot C:\Windows /s S: /f UEFI|BIOS|ALL` : especifica un tipo de firmware al copiar los archivos de arranque a un volumen especificado
