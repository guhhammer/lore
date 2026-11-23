# jj workspace

Beheer Jujutsu-werkruimtes.

## Usage
- `jj workspace list` : toon alle werkruimtes die aan de repository gekoppeld zijn
- `jj workspace add pad/naar/map` : maak een nieuwe werkruimte aan op het opgegeven pad
- `jj workspace add --name werkruimte_naam -r revisie pad/naar/map` : maak een nieuwe werkruimte aan met een specifieke naam en bovenliggende revisie
- `jj workspace root` : toon de hoofdmap van de huidige werkruimte
- `jj workspace root --name werkruimte_naam` : toon de hoofdmap van een specifieke werkruimte
- `jj workspace rename nieuwe_naam` : hernoem de huidige werkruimte
- `jj workspace forget werkruimte_naam` : stop met het volgen van een werkruimte zonder de bestanden te verwijderen
- `jj workspace update-stale` : werk een werkruimte bij waarvan de werkkopie verouderd is

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-workspace
