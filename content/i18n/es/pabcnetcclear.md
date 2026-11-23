# pabcnetcclear

Preprocesa y compila archivos fuente de PascalABC.NET.

## Usage
- `pabcnetcclear ruta\al\archivo_fuente.pas` : compila el archivo fuente especificado en un ejecutable con el mismo nombre
- `pabcnetcclear /Output:ruta\al\_archivo.exe ruta\al\archivo_fuente.pas` : compila el archivo fuente especificado en un ejecutable con el nombre especificado
- `pabcnetcclear /Debug:0|1 ruta\al\archivo_fuente.pas` : compila el archivo fuente especificado en un ejecutable con el mismo nombre junto con/sin información de depuración
- `pabcnetcclear /SearchDir:ruta\al\directorio ruta\al\archivo_fuente.pas` : permite que las unidades se busquen en la ruta especificada mientras se compila el archivo fuente en un ejecutable con el mismo nombre
- `pabcnetcclear /Define:símbolo ruta\al\archivo_fuente.pas` : compila el archivo fuente especificado en un ejecutable, definiendo un símbolo

## Links
- More information: https://pascalabc.net/
