# mamba

Snelle, platformonafhankelijke pakketbeheerder, bedoeld als drop-in vervanging voor conda. Sommige subcommando's zoals `repoquery` hebben hun eigen documentatie.

## Usage
- `mamba create -n omgevingsnaam python=3.10 matplotlib` : maak een nieuwe omgeving aan en installeer hierin de opgegeven pakketten
- `mamba install -c conda-forge python=3.6 numpy` : installeer pakketten in de huidige omgeving, met opgave van het pakketkanaal
- `mamba update -a` : update alle pakketten in de huidige omgeving
- `mamba repoquery search numpy` : zoek naar een specifiek pakket in alle repositories
- `mamba info -e` : toon alle omgevingen
- `mamba clean -pt` : verwijder ongebruikte pakketten en tarballs uit de cache
- `mamba activate omgevingsnaam` : activeer een omgeving
- `mamba list` : toon alle geïnstalleerde pakketten in de momenteel geactiveerde omgeving

## Links
- More information: https://mamba.readthedocs.io/en/latest/user_guide/mamba.html
