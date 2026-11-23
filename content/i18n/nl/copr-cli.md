# copr-cli

Interface met Fedora-projecten copr instantie voor het bouwen van RPM's en het publiceren ervan.

## Usage
- `copr-cli whoami` : toon de gebruiker ingelogd in copr
- `copr-cli build repository pad/naar/spec_bestand` : bouw een lokaal spec-bestand op copr
- `copr-cli list-builds repository` : controleer de status van de builds
- `copr-cli buildscm repository --clone-url https://git.example.org/repo --spec spec_bestandsnaam` : trigger een copr build van een spec-bestand vanuit een publieke (Git) repository

## Links
- More information: https://manned.org/copr-cli
