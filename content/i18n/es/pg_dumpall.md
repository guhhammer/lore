# pg_dumpall

Extrae un grupo de bases de datos PostgreSQL en un archivo de script u otro archivo de almacenamiento.

## Usage
- `pg_dumpall >ruta/al/archivo.sql` : vuelca todas las bases de datos
- `pg_dumpall -U usuario >ruta/al/archivo.sql` : vuelca todas las bases de datos utilizando un nombre de usuario específico
- `pg_dumpall -h equipo -p puerto >archivo_resultado.sql` : lo mismo que antes, usando un equipo y puerto
- `pg_dumpall -a >ruta/al/archivo.sql` : recupera solo datos de las bases de datos en un archivo script-SQL
- `pg_dumpall -s >archivo_resultado.sql` : vuelca solo el esquema (definiciones de datos) en un archivo script-SQL

## Links
- More information: https://www.postgresql.org/docs/current/app-pg-dumpall.html
