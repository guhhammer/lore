# ceph

Un sistema de almacenamiento unificado.

## Usage
- `ceph status` : comprueba el estado del cluster
- `ceph df` : comprueba las estadísticas de uso del cluster
- `ceph pg dump --format plain` : obtiene las estadísticas de los grupos de colocación de un cluster
- `ceph osd pool create nombre_pool número_de_página` : crea un grupo de almacenamiento
- `ceph osd pool delete nombre_pool` : elimina un pool de almacenamiento
- `ceph osd pool rename nombre_actual nombre_nuevo` : cambia el nombre de un pool de almacenamiento
- `ceph pg repair nombre_pool` : auto-repara pool de almacenamiento

## Links
- More information: https://docs.ceph.com/en/latest/man/8/ceph/
