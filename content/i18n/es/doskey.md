# doskey

Administra macros, comandos de Windows y líneas de comandos.

## Usage
- `doskey /macros` : lista macros disponibles
- `doskey nombre = "comando"` : crea una nueva macro
- `doskey /exename=nombre_del_ejecutable nombre = "comando"` : crea una nueva macro para un ejecutable específico
- `doskey nombre =` : elimina una macro
- `doskey /history` : muestra todos los comandos que están almacenados en memoria
- `doskey /macros >ruta\al\archivo_macinit` : guarda macros en un archivo para portabilidad
- `doskey /macrofile = ruta\al\archivo_macinit` : carga macros desde un archivo
