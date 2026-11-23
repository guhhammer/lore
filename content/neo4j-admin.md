# neo4j-admin

Manage and administer a Neo4j DBMS (Database Management System).

## Usage
- `neo4j-admin server start` : start the DBMS
- `neo4j-admin server stop` : stop the DBMS
- `neo4j-admin dbms set-initial-password database_name` : set the initial password of the default `neo4j` user (prerequisite for the first start of the DBMS)
- `neo4j-admin database dump --to-path=path/to/directory database_name` : create an archive (dump) of an offline database to a file named `database_name.dump`
- `neo4j-admin database load --from-path=path/to/directory database_name --overwrite-destination=true` : load a database from an archive named `database_name.dump`
- `neo4j-admin <path/to/file.dump database load --from-stdin database_name --overwrite-destination=true` : load a database from a specified archive file through `stdin`
- `neo4j-admin --help` : display help

## See also
- cypher-shell
- mysqld

## Categories
files, databases, monitoring

## Links
- More information: https://neo4j.com/docs/operations-manual/current/neo4j-admin-neo4j-cli/
- tldr page: https://tldr.inbrowser.app/pages/common/neo4j-admin
