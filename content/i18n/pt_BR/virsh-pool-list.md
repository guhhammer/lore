# virsh pool-list

Lista informações sobre pools de armazenamento de máquinas virtuais.

## Usage
- `virsh pool-list` : lista o nome, estado e se a inicialização automática está habilitada ou desabilitada para pools de armazenamento ativos
- `virsh pool-list --all|inactive` : lista informações para pools de armazenamento ativos e inativos ou apenas inativos
- `virsh pool-list --details` : lista informações estendidas sobre persistência, capacidade, alocação e espaço disponível para pools de armazenamento ativos
- `virsh pool-list --autostart|no-autostart` : lista informações para pools de armazenamento ativos com inicialização automática habilitada ou desabilitada
- `virsh pool-list --persistent|transient` : lista informações para pools de armazenamento ativos que são persistentes ou transitórios
- `virsh pool-list --name --uuid` : lista o nome e UUID dos pools de armazenamento ativos

## Links
- More information: https://manned.org/virsh
