# restorecon

Restaura el contexto de seguridad de SELinux en archivos y directorios según las reglas persistentes.

## Usage
- `ls -dlZ ruta/al/archivo_o_directorio` : muestra el contexto de seguridad actual de un archivo o directorio
- `restorecon ruta/al/archivo_o_directorio` : restaura el contexto de seguridad de un archivo o directorio
- `restorecon -R -v ruta/al/directorio` : restaura el contexto de seguridad de un directorio de forma recursiva y muestra todas las etiquetas modificadas
- `restorecon -R -T 0 -p ruta/al/directorio` : restaura el contexto de seguridad de un directorio de forma recursiva, utilizando todos los subprocesos disponibles, y muestra el progreso
- `restorecon -R -n -v ruta/al/directorio` : previsualiza los cambios en las etiquetas que se producirían sin aplicarlos

## Links
- More information: https://manned.org/restorecon
