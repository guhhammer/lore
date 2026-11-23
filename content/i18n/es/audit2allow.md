# audit2allow

Analiza los registros en busca de mensajes relativos a permisos denegados. Genera un informe de las reglas de Type Enforcement (TE) que podrían permitir operaciones con éxito.

## Usage
- `audit2allow -a` : muestra todos los mensajes generados en los registros de auditoría y mensajes
- `audit2allow -b` : muestra todos los mensajes generados desde el último arranque
- `audit2allow -e` : muestra información detallada sobre los mensajes generados
- `audit2allow -v` : activa el modo de salida detallada
- `audit2allow -R` : utiliza macros instaladas para generar una política de referencia
- `audit2allow -p ruta/al/archivo_de_directiva` : especifica un archivo de política para su posterior análisis
- `audit2allow -t tipo_regex` : limita el análisis a los mensajes con un tipo especificado en `regex`
- `audit2allow -h` : muestra la ayuda

## Links
- More information: https://manned.org/audit2allow
