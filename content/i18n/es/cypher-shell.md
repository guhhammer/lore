# cypher-shell

Abre una sesión interactiva y ejecuta consultas Cypher contra una instancia Neo4j.

## Usage
- `cypher-shell` : se conecta a una instancia local en el puerto por defecto (`neo4j://localhost:7687`)
- `cypher-shell --address neo4j://host:puerto` : se conecta a una instancia remota
- `cypher-shell --username usuario --password contraseña` : se conecta y proporciona credenciales de seguridad
- `cypher-shell --database nombre_base_de_datos` : se conecta a una base de datos específica
- `cypher-shell --file ruta/al/archivo.cypher` : ejecuta sentencias Cypher en un archivo y lo cierra
- `cypher-shell --log ruta/al/archivo.log` : activa el registro en un archivo
- `cypher-shell --help` : muestra la ayuda

## Links
- More information: https://neo4j.com/docs/operations-manual/current/cypher-shell/
