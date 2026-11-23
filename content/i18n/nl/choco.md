# choco

De Chocolatey pakketmanager. Sommige subcommando's zoals `install`, `upgrade`, `pin`, etc. hebben hun eigen documentatie.

## Usage
- `choco install pakket_naam` : installeer een pakket
- `choco upgrade pakket_naam` : upgrade een specifiek geïnstalleerd pakket
- `choco upgrade all -y` : upgrade alle verouderde pakketten en bevestig automatisch alle vragen
- `choco uninstall pakket_naam -y` : verwijder een pakket en bevestig automatisch alle vragen
- `choco search query` : zoek naar pakketten op naam of sleutelwoord
- `choco list` : toon alle op de machine geïnstalleerde pakketten
- `choco outdated` : toon pakketten die verouderd zijn
- `choco install pakket_naam -s bron` : installeer een pakket van een specifieke bron

## Links
- More information: https://docs.chocolatey.org/en-us/choco/commands/
