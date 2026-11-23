# neo4j-admin

Gestiona y administra un Neo4j DBMS (Sistema de Gestión de Bases de Datos).

## Usage
- `neo4j-admin server start` : inicia el DBMS
- `neo4j-admin server stop` : detiene el DBMS
- `neo4j-admin dbms set-initial-password nombre_base_de_datos` : establece la contraseña inicial del usuario predeterminada `neo4j` (requisito para el primer arranque del DBMS)
- `neo4j-admin database dump --to-path=ruta/al/directorio nombre_de_base_de_datos` : crea un archivo con una base de datos sin conexión llamado `nombre_base_de_datos.dump`
- `neo4j-admin database load --from-path=ruta/al/directorio nombre_de_base_de_datos --overwrite-destination=true` : carga una base de datos desde un archivo llamado `nombre_base_de_datos.dump`
- `neo4j-admin <ruta/a/nombre_archivo.dump database load --from-stdin nombre_de_base_de_datos --overwrite-destination=true` : carga una base de datos desde un archivo especificado a través de `stdin`
- `neo4j-admin --help` : muestra la ayuda

## Links
- More information: https://neo4j.com/docs/operations-manual/current/neo4j-admin-neo4j-cli/
