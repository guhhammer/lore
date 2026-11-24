# apptainer

Manage containers for HPC and scientific computing. Some subcommands such as `build`, `pull`, `push`, etc. have their own usage documentation.

## Usage
- `apptainer pull path/to/image.sif docker://image:tag` : download a container from Docker Hub
- `apptainer pull path/to/image.sif library://user/collection/container:tag` : download a container from the Container Library
- `apptainer build path/to/image.sif path/to/definition.def` : build a container from a definition file
- `apptainer shell path/to/image.sif` : start an interactive shell inside a container
- `apptainer exec path/to/image.sif command` : execute a command inside a container
- `apptainer run path/to/image.sif` : run the default runscript of a container
- `apptainer inspect path/to/image.sif` : inspect a container's metadata
- `apptainer -h` : display help

## Categories
containers, files, media

## Links
- More information: https://apptainer.org/docs/user/main/cli.html
- tldr page: https://tldr.inbrowser.app/pages/linux/apptainer
- Wikipedia: https://en.wikipedia.org/wiki/Apptainer
