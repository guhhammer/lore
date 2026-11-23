# pg_ctl

Utilidad para controlar un servidor PostgreSQL y un grupo (clúster) de bases de datos.

## Usage
- `pg_ctl -D directorio_de_datos init` : inicia un nuevo grupo de base de datos PostgreSQL
- `pg_ctl -D directorio_de_datos start` : inicia un servidor PostgreSQL
- `pg_ctl -D directorio_de_datos stop` : detiene un servidor PostgreSQL
- `pg_ctl -D directorio_de_datos restart` : reinicia un servidor PostgreSQL
- `pg_ctl -D directorio_de_datos reload` : recarga la configuración del servidor PostgreSQL

## Links
- More information: https://www.postgresql.org/docs/current/app-pg-ctl.html
