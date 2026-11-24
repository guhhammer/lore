# apptainer pull

Pull container images from remote sources.

## Usage
- `apptainer pull path/to/image.sif docker://image:tag` : pull a container from Docker Hub
- `apptainer pull path/to/image.sif library://user/collection/container:tag` : pull a container from the Container Library
- `apptainer pull path/to/image.sif oras://registry/namespace/image:tag` : pull a container from an OCI registry
- `apptainer pull --arch amd64|arm64|ppc64le path/to/image.sif library://image:tag` : pull a container for a specific architecture
- `apptainer pull -F path/to/image.sif docker://image:tag` : force overwrite an existing image file
- `apptainer pull --sandbox path/to/directory docker://image:tag` : pull a container as a writable sandbox directory
- `apptainer pull --disable-cache path/to/image.sif docker://image:tag` : pull a container without using the cache
- `apptainer pull -h` : display help

## See also
- apptainer-push

## Categories
containers, media, files

## Links
- More information: https://apptainer.org/docs/user/main/cli/apptainer_pull.html
- tldr page: https://tldr.inbrowser.app/pages/linux/apptainer-pull
- Wikipedia: https://en.wikipedia.org/wiki/Apptainer
