# cypher-shell

Open an interactive session and run Cypher queries against a Neo4j instance.

## Usage
- `cypher-shell` : connect to a local instance on the default port (`neo4j://localhost:7687`)
- `cypher-shell --address neo4j://host:port` : connect to a remote instance
- `cypher-shell --username username --password password` : connect and supply security credentials
- `cypher-shell --database database_name` : connect to a specific database
- `cypher-shell --file path/to/file.cypher` : execute Cypher statements in a file and close
- `cypher-shell --log path/to/file.log` : enable logging to a file
- `cypher-shell --help` : display help

## See also
- neo4j-admin
- mysql

## Categories
shell, files, network

## Links
- More information: https://neo4j.com/docs/operations-manual/current/cypher-shell/
- tldr page: https://tldr.inbrowser.app/pages/common/cypher-shell
