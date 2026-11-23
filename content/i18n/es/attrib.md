# attrib

Muestra o cambia los atributos de archivos o directorios.

## Usage
- `attrib` : muestra todos los atributos establecidos de los archivos en el directorio actual
- `attrib ruta\al\directorio` : muestra todos los atributos establecidos de los archivos en un directorio específico
- `attrib /d` : muestra todos los atributos establecidos de archivos y directorios en el directorio actual
- `attrib /s` : muestra todos los atributos establecidos de los archivos en el directorio actual y subdirectorios
- `attrib +r|a|s|h|i ruta\al\archivo_o_directorio1 ruta\al\archivo_o_directorio2 ...` : agrega el atributo `solo-lectura` o `archivo` o `sistema` o `ocultoh` o `contenido no indexado` a archivos o directorios
- `attrib -r|a|s|h|i ruta\al\archivo_o_directorio1 ruta\al\archivo_o_directorio2 ...` : elimina un atributo específico de archivos o directorios
