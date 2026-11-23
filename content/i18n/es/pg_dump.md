# pg_dump

Extrae una base de datos PostgreSQL en un archivo de script u otro archivo de almacenamiento.

## Usage
- `pg_dump nombre_base_de_datos >archivo_resultado.sql` : vuelca la base de datos en un archivo script-SQL
- `pg_dump -U usuario nombre_base_de_datos >archivo_resultado.sql` : igual que antes usando además un nombre de usuario
- `pg_dump -h equipo -p puerto nombre_base_de_datos >archivo_resultado.sql` : lo mismo que antes usando además equipo y puerto
- `pg_dump -F c nombre_base_de_datos >archivo_resultado.dump` : vuelca una base de datos en un archivo de formato personalizado
- `pg_dump -a nombre_base_de_datos >ruta/al/archivo_resultado.sql` : recupera solo datos de bases de datos en un archivo script-SQL
- `pg_dump -s nombre_base_de_datos >ruta/al/archivo_resultado.sql` : vuelca solo el esquema (definiciones de datos) en un archivo script-SQL

## Links
- More information: https://www.postgresql.org/docs/current/app-pgdump.html
