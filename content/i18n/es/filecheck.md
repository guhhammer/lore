# filecheck

Verificador de archivos de coincidencia de patrones flexible. Se utiliza típicamente a partir de pruebas de regresión LLVM y forma parte de una prueba `lit`.

## Usage
- `FileCheck --input-file=ruta/al/archivo_de_entrada ruta/al/archivo_de_comprobación` : compara el contenido de `archivo_entrada` con el archivo de patrones `archivo_comprobado`
- `echo "algún_texto" | FileCheck ruta/al/archivo_de_comprobación` : busca coincidencias de `stdin` con el archivo de patrones `archivo_de_comprobación`
- `echo "algún_texto" | FileCheck --check-prefix=prefijo ruta/al/archivo_comprobado` : busca coincidencias con el `prefijo` de comprobación personalizado especificado (Nota: el prefijo predeterminado es `CHECK`)
- `echo "some_text" | FileCheck -v ruta/al/archivo_comprobado` : imprime las coincidencias de patrón de directivas
- `llvm-as ruta/al/código_llvm_.ll | FileCheck ruta/al/archivo_comprobado` : introduce `llvm_code.ll` en llvm-as y, a continuación, envía la salida a FileCheck para que coincida

## Links
- More information: https://llvm.org/docs/CommandGuide/FileCheck.html
