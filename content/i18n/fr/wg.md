# wg

Gestion de la configuration des interfaces WireGuard.

## Usage
- `sudo wg` : vérifie l'état des interfaces actuellement actives
- `wg genkey` : génère une clé privée
- `wg <chemin/vers/clé_privée pubkey >chemin/vers/clé_publique` : génère une clé publique à partir d'une clé privée
- `wg genkey | tee chemin/vers/clé_privée | wg pubkey >chemin/vers/clé_publique` : génère une clé publique et privée
- `sudo wg showconf wg0` : affiche la configuration actuelle d'une interface wireguard

## Links
- More information: https://www.wireguard.com/quickstart/
