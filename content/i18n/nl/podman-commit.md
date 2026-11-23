# podman commit

Maak een nieuwe image op basis van de gewijzigde container.

## Usage
- `podman commit container image:tag` : maak een image van een specifieke container
- `podman commit -c "ENV naam=waarde" container image:tag` : pas een `ENV`-instructie toe op de aangemaakte image
- `podman commit -c CMD=commando -c ENTRYPOINT=commando -c "LABEL sleutel=waarde" container image:tag` : pas `LABEL`-, `ENTRYPOINT`- en `CMD`-instructies toe op de aangemaakte image
- `podman commit -a "auteur" -m "opmerking" container image:tag` : maak een image met een specifieke auteur en opmerking in de metadata
- `podman commit -p false container image:tag` : maak een image zonder de container te pauzeren tijdens het committen
- `podman commit --help` : toon de help

## Links
- More information: https://docs.podman.io/en/latest/markdown/podman-commit.1.html
