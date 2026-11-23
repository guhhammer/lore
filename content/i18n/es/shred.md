# shred

Sobrescribe archivos para eliminar datos de forma segura.

## Usage
- `shred ruta/al/archivo` : sobrescribe un archivo
- `shred -v ruta/al/archivo` : sobrescribe un archivo y muestra el progreso en pantalla
- `shred -z ruta/al/archivo` : sobrescribe un archivo, sustituyendo los datos aleatorios por ceros
- `shred -n 25 ruta/al/archivo` : sobrescribe un archivo un número específico de veces
- `shred -u ruta/al/archivo` : sobrescribe un archivo y lo elimina
- `shred -vzu -n 100 ruta/al/archivo` : sobrescribe un archivo 100 veces, añade una sobrescritura final con ceros, elimina el archivo tras sobrescribirlo y muestra el progreso detallado en pantalla
