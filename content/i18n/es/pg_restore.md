# pg_restore

Restaura una base de datos PostgreSQL de un archivo creado con pg_dump.

## Usage
- `pg_restore -d nombre_base_de_datos archivo_de_datos.dump` : restaura un archivo en una base de datos existente
- `pg_restore -U usuario -d nombre_base_de_datos archivo_de_datos.dump` : igual que antes, utilizando un nombre de usuario
- `pg_restore -h equipo -p puerto -d nombre_base_de_datos archivo_de_datos.dump` : lo mismo que antes, usando un nombre de equipo y puerto
- `pg_restore -l archivo_de_datos.dump` : lista los objetos de bases de datos incluidos en el archivo
- `pg_restore -c -d nombre_base_de_datos archivo_de_datos.dump` : limpia los objetos de base de datos antes de crearlos
- `pg_restore -j 2 -d nombre_base_de_datos archivo_de_datos.dump` : utiliza múltiples trabajos para hacer la restauración

## Links
- More information: https://www.postgresql.org/docs/current/app-pgrestore.html
