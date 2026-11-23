# elasticsearch-node

Manage low-level Elasticsearch node operations such as shutdown, repurpose, or viewing info.

## Usage
- `elasticsearch-node info` : display information about the current node
- `elasticsearch-node unsafe-bootstrap` : prepare the node for a full cluster restart (e.g., after upgrading)
- `elasticsearch-node repurpose` : repurpose a node for a different role (e.g., from master to data node)
- `elasticsearch-node roles` : list the roles assigned to the node
- `elasticsearch-node diagnostics` : show the installed JVM version, Elasticsearch home path, and other diagnostic information
- `elasticsearch-node -h` : display help

## Categories
system

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/elasticsearch-node
