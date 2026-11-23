# cgexec

Limita, mede e controla recursos usados pelos processos. Há vários tipos de cgroup (conhecidos como controladores), tal como `cpu`, `memory`, etc.

## Usage
- `cgexec -g controlador:nome_cgroup nome_processo` : executa um processo em um cgroup e controlador providos pelo usuário

## Links
- More information: https://manned.org/cgexec
