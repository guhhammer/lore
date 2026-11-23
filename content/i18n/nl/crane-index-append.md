# crane index append

Voeg een manifest toe aan een remote index. Dit subcommando pusht een index op basis van een (optionele) basisindex, met toegevoegde manifests. Het platform voor toegevoegde manifests wordt afgeleid van het configuratiebestand of weggelaten als dat niet haalbaar is.

## Usage
- `crane index append` : voeg een manifest toe aan een remote index
- `crane index append -m manifest_naam1 manifest_naam2 ...` : voeg specifieke manifests toe aan de basisindex
- `crane index append -t tag_naam` : pas een tag toe op de resulterende image
- `crane index append --docker-empty-base` : gebruik Docker-media in plaats van OCI voor een lege basisindex
- `crane index append --flatten` : voeg elk van zijn kinderen toe in plaats van de index zelf (standaard waar)
- `crane index append -h` : toon de help
