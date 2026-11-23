# crane append

Push een image gebaseerd op een (optionele) basisimage. Voegt lagen toe met de inhoud van de opgegeven tarballs.

## Usage
- `crane append -b image_naam` : push een image gebaseerd op een basisimage
- `crane append -f layer_naam1 layer_naam2 ...` : push een image met een toegevoegde laag vanuit een tarball
- `crane append -t tag_naam` : push een image met een toegevoegde laag met een nieuwe tag
- `crane append -o pad/naar/tarball` : push de resulterende image naar een nieuwe tarball
- `crane append --oci-empty-base` : gebruik een lege basisimage van het type OCI-media in plaats van Docker
- `crane append --set-base-image-annotations` : annoteer de resulterende image als gebaseerd op de basisimage
- `crane append -h` : toon de help
