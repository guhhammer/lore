# nixos-rebuild

Riconfigura una macchina NixOS.

## Usage
- `sudo nixos-rebuild switch` : compila e passa alla nuova configurazione, rendendola quella predefinita all'avvio
- `sudo nixos-rebuild switch -p nome` : compila e passa alla nuova configurazione, rendendola quella predefinita all'avvio e assegnando un nome alla voce del menù di avvio
- `sudo nixos-rebuild switch --upgrade` : compila e passa alla nuova configurazione, rendendola quella predefinita all'avvio e installando gli aggiornamenti
- `sudo nixos-rebuild switch --rollback` : annulla le modifiche alla configurazione, passando alla generazione precedente
- `sudo nixos-rebuild boot` : compila la nuova configurazione rendendola quella predefinita all'avvio, senza passare ad essa
- `sudo nixos-rebuild test` : compila e passa alla nuova configurazione senza aggiungere la voce al menù di avvio (a scopo di test)
- `sudo nixos-rebuild build-vm` : compila la configurazione e la apre in una macchina virtuale
- `nixos-rebuild list-generations` : elenca le generazioni disponibili in modo simile al menù di avvio

## Links
- More information: https://nixos.org/nixos/manual/#sec-changing-config
