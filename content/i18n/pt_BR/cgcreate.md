# cgcreate

Cria cgroups, usados para limitar, medir e controlar recursos usados pelos processos. Tipos de `cgroups` podem ser `memory`, `cpu`, `net_cls`, etc.

## Usage
- `cgcreate -g tipo_grupo:nome_grupo` : cria um novo grupo
- `cgcreate -g tipo_grupo1,tipo_grupo2:nome_grupo` : cria um novo grupo com vários tipos de cgroup
- `mkdir /sys/fs/cgroup/tipo_grupo2/nome_grupo/nome_subgrupo` : cria um subgrupo

## Links
- More information: https://manned.org/cgcreate
