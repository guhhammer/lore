# nixos-container

Avvia container NixOS usando container Linux.

## Usage
- `sudo nixos-container list` : elenca i container in esecuzione
- `sudo nixos-container create nome_container --config-file percorso_file_configurazione_nix` : crea un container NixOS con un file di configurazione specifico
- `sudo nixos-container start|stop|terminate|destroy|status nome_container` : avvia, ferma, termina o elimina uno specifico container
- `sudo nixos-container run nome_container -- comando argomenti` : esegue un comando all'interno di un container in esecuzione
- `sudo $EDITOR /var/lib/container/nome_container/etc/nixos/configuration.nix && sudo nixos-container update nome_container` : aggiorna la configurazione di un container
- `sudo nixos-container root-login nome_container` : entra in una sessione shell interattiva in un container già in esecuzione

## Links
- More information: https://nixos.org/manual/nixos/stable/#ch-containers
