# lxc-create

Crea container Linux.

## Usage
- `sudo lxc-create -n nome_container -t download` : crea un container interattivamente in `/var/lib/lxc/`
- `sudo lxc-create -P /percorso/della/directory/ -n nome_container -t download` : crea un container in una directory di destinazione
- `sudo lxc-create -n nome_container -t download -- -d nome_distro -r versione_release -a architettura` : crea un container passando opzioni a un template
- `lxc-create -?` : mostra l'aiuto

## Links
- More information: https://linuxcontainers.org/lxc/getting-started/
