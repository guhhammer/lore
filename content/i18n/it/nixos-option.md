# nixos-option

Ispeziona una configurazione NixOS.

## Usage
- `nixos-option opzione` : elenca tutte le sottochiavi di una data opzione
- `nixos-option boot.kernelModules` : elenca tutti i moduli del kernel corrente
- `nixos-option users.users.nome_utente.openssh.authorizedKeys.keyFiles|keys` : elenca le chiavi di autorizzazioni per uno specifico utente
- `nixos-option nix.buildMachines` : elenca tutte le macchine di compilazione remote
- `NIXOS_CONFIG=percorso_per_configuration.nix nixos-option opzione` : elenca tutte le sottochiavi di una data opzione di un'altra configurazione NixOS
- `nixos-option -r users.users.utente` : visualizza ricorsivamente tutti i valori per un utente

## Links
- More information: https://nixos.org/manual/nixos/stable/index.html#sec-modularity
