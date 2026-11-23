# usql

Interfaz de línea de comandos universal para bases de datos SQL.

## Usage
- `usql sqlserver|mysql|postgres|sqlite3|...://usuario:contraseña@host:puerto/nombre_base_de_datos` : conecta a una base de datos específica
- `usql -f ruta/al/query.sql` : ejecuta comandos desde un archivo
- `usql -c "comando_sql"` : ejecuta un comando SQL específico
- `comando` : [Interactivo] Ejecuta un comando SQL en el intérprete de comandos de `usql`
- `\d` : [Interactivo] Muestra el esquema de la base de datos
- `\g ruta/al/archivo_con_resultados` : [Interactivo] Exporta los resultados de la consulta a un archivo específico
- `\copy ruta/a/datos.csv nombre_de_tabla` : [Interactivo] Importa datos de un archivo CSV a una tabla específica

## Links
- More information: https://github.com/xo/usql#using
