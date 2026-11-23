# ntfs-read.py

Un explorador NTFS de solo lectura para acceder y extraer archivos de volúmenes NTFS. Forma parte del paquete Impacket.

## Usage
- `ntfs-read.py volumen` : abre un volumen NTFS para explorarlo (por ejemplo, `C:\.\\` o `/dev/disk1s1`)
- `ntfs-read.py -extract \windows\system32\config\sam volumen` : extrae un archivo específico de un volumen NTFS (por ejemplo, `\windows\system32\config\sam`)
- `ntfs-read.py -debug volumen` : habilita la salida de depuración
- `ntfs-read.py --help` : muestra la ayuda

## Links
- More information: https://github.com/fortra/impacket
