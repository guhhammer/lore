# findstr

Busca texto especificado dentro de uno o más archivos.

## Usage
- `findstr "cadena1 cadena2 ..." *` : busca una o más cadenas en todos los archivos
- `dir | findstr "cadena1 cadena2 ..."` : busca una o más cadenas en la salida de un comando canalizado
- `findstr /s "cadena1 cadena2 ..." *` : busca una o más cadenas en todos los archivos de forma recursiva
- `findstr /i "cadena1 cadena2 ..." *` : busca cadenas utilizando una búsqueda sin distinguir entre mayúsculas y minúsculas
- `findstr /r "expresión" *` : busca cadenas en todos los archivos utilizando expresiones regulares
- `findstr /c:"cadena1 cadena2 ..." *.txt` : busca una cadena literal (que contenga espacios) en todos los archivos de texto
- `findstr /n "cadena1 cadena2 ..." *` : muestra el número de línea antes de cada línea coincidente
- `findstr /m "cadena1 cadena2 ..." *` : muestra solo los nombres de archivo que contienen una coincidencia
