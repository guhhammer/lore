# pngtopam

Convierte una imagen PNG a una imagen Netpbm.

## Usage
- `pngtopam ruta/a/la/imagen.png >ruta/al/resultado.pam` : convierte la imagen PNG especificada en imagen Netpbm
- `pngtopam -alphapam ruta/a/la/imagen.png >ruta/al/resultado.pam` : crea una imagen de salida que incluye tanto la imagen principal como la máscara de transparencia de la imagen de entrada
- `pngtopam -m -ba color ruta/a/la/imagen.png >ruta/al/resultado.pam` : reemplaza píxeles transparentes por el color especificado
- `pngtopam -te ruta/al/archivo.txt ruta/a/la/imagen.png >ruta/al/resultado.pam` : escribe los trozos de tEXt encontrados en la imagen de entrada al archivo de texto especificado

## Links
- More information: https://netpbm.sourceforge.net/doc/pngtopam.html
