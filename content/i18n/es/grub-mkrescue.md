# grub-mkrescue

Crea una imagen arrancable de GRUB para CD/USB/disquete.

## Usage
- `grub-mkrescue --output grub.iso .` : crea una ISO arrancable desde el directorio actual y la guarda como `grub.iso`
- `grub-mkrescue --directory /usr/lib/grub/i386-pc --output grub.iso ruta/al/origen` : crea una ISO usando archivos de GRUB desde un directorio personalizado
- `grub-mkrescue --compress no|xz|gz|lzo --output grub.iso ruta/al/origen` : usa compresión para los archivos de GRUB al construir la imagen, establecer `no` desactiva la compresión
- `grub-mkrescue --disable-cli --output grub.iso ruta/al/origen` : desactiva la interfaz de línea de comandos de GRUB en la imagen generada
- `grub-mkrescue --modules "part_gpt iso9660" --output grub.iso ruta/al/origen` : precarga módulos específicos de GRUB en la imagen
- `grub-mkrescue --output grub.iso -- -volid nombre_volumen ruta/al/origen` : pasa opciones adicionales directamente a `xorriso`
- `grub-mkrescue -?` : muestra la ayuda
- `grub-mkrescue --version` : muestra la versión
