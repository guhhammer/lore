# apptainer push

Push container images to remote registries.

## Usage
- `apptainer push path/to/image.sif library://user/collection/container:tag` : push a container to the Container Library
- `apptainer push path/to/image.sif oras://registry/namespace/image:tag` : push a container to an OCI registry
- `apptainer push -U path/to/image.sif library://user/collection/container:tag` : push an unsigned container (skip signature verification)
- `apptainer push -D "description" path/to/image.sif library://user/collection/container:tag` : push a container with a description (library only)
- `apptainer push -h` : display help

## See also
- apptainer-pull

## Categories
containers, media, development

## Links
- More information: https://apptainer.org/docs/user/main/cli/apptainer_push.html
- tldr page: https://tldr.inbrowser.app/pages/linux/apptainer-push
