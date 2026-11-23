# apptainer build

Build Apptainer container images.

## Usage
- `apptainer build path/to/image.sif path/to/definition.def` : build a container from a definition file
- `apptainer build path/to/image.sif docker://image:tag` : build a container from Docker Hub
- `apptainer build path/to/image.sif library://user/collection/container:tag` : build a container from the Container Library
- `apptainer build -s path/to/directory docker://image:tag` : build a writable sandbox directory instead of an image file
- `apptainer build --disable-cache path/to/image.sif docker://image:tag` : build a container without using the cache
- `apptainer build -F path/to/image.sif path/to/definition.def` : force overwrite an existing image file
- `apptainer build -f path/to/image.sif path/to/definition.def` : build using fakeroot for unprivileged builds
- `apptainer build -h` : display help

## Categories
containers, development, files

## Links
- More information: https://apptainer.org/docs/user/main/cli/apptainer_build.html
- tldr page: https://tldr.inbrowser.app/pages/linux/apptainer-build
