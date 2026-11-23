# lxc network

Gestisce le reti per i container LXD.

## Usage
- `lxc network list` : elenca tutte le reti disponibili
- `lxc network show nome_network` : mostra la configurazione di una rete specifica
- `lxc network attach nome_network nome_container` : aggiunge un'istanza in esecuzione a una rete specifica
- `lxc network create nome_network` : crea una nuova rete gestita
- `lxc network set nome_network bridge.external_interfaces eth0` : imposta un'interfaccia bridge di una rete specifica
- `lxc network set nome_network ipv4.nat false` : disabilita NAT per una rete specifica

## Links
- More information: https://canonical.com/lxd/docs/latest/reference/manpages/lxc/network/
