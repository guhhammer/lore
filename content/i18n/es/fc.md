# fc

Compara las diferencias entre dos archivos o conjuntos de archivos. Usa comodines (*) para comparar conjuntos de archivos.

## Usage
- `fc ruta\al\archivo1 ruta\al\archivo2` : compara 2 archivos especificados
- `fc /c ruta\al\archivo1 ruta\al\archivo2` : realiza una comparación sin distinguir entre mayúsculas y minúsculas
- `fc /u ruta\al\archivo1 ruta\al\archivo2` : compara archivos como texto Unicode
- `fc /l ruta\al\archivo1 ruta\al\archivo2` : compara archivos como texto ASCII
- `fc /b ruta\al\archivo1 ruta\al\archivo2` : compara archivos como binarios
- `fc /t ruta\al\archivo1 ruta\al\archivo2` : deshabilita la expansión de tabulaciones a espacios
- `fc /w ruta\al\archivo1 ruta\al\archivo2` : comprime espacios en blanco (tabulaciones y espacios) para hacer comparaciones
