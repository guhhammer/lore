# fabric

Un marco de código abierto para mejorar las capacidades humanas mediante la IA. Proporciona un marco modular para resolver problemas específicos utilizando un conjunto de indicaciones de IA obtenidas mediante crowdsourcing.

## Usage
- `fabric -S` : ejecuta la instalación para configurar fabric
- `fabric -l` : muestra todos los patrones disponibles
- `fabric <ruta/al/archivo_de_entrada -p nombre_del_patrón` : ejecuta un patrón con la entrada de un archivo
- `fabric -y "https://www.youtube.com/watch?v=video_id" -p nombre_del_patrón` : ejecuta un patrón en una dirección URL de YouTube
- `fabric -p pattern1 | fabric -p pattern2` : encadena patrones conectando la salida de uno al otro
- `fabric -p nombre_patrón_personalizado` : ejecuta un patrón personalizado definido por el usuario
- `fabric -p nombre_del_patrón -o ruta/al/archivo_de_salida` : ejecuta un patrón y guarda la salida en un archivo
- `fabric -p nombre_del_patrón -v "nombre_variable:valor"` : ejecuta un patrón con las variables especificadas

## Links
- More information: https://github.com/danielmiessler/fabric#usage
