# virsh connect

Conectar-se a um hipervisor de máquina virtual.

## Usage
- `virsh connect` : conecta ao hipervisor padrão
- `virsh connect qemu:///system` : conecta como root ao hipervisor local QEMU/KVM
- `virsh connect qemu:///session` : inicia uma nova instância do hipervisor e conectar-se a ela como usuário local
- `virsh connect qemu+ssh://nome_do_usuário@nome_do_host/system` : conecta como root a um hipervisor remoto usando SSH

## Links
- More information: https://manned.org/virsh
