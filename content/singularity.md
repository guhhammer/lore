# singularity

Manage Singularity containers and images.

## Usage
- `singularity pull --name image.sif library://godlovedc/funny/lolcow:latest` : download a remote image from Sylabs Cloud
- `singularity build image.sif docker://godlovedc/lolcow` : rebuild a remote image using the latest Singularity image format
- `singularity shell image.sif` : start a container from an image and get a shell inside it
- `singularity exec image.sif command` : start a container from an image and run a command
- `singularity run image.sif` : start a container from an image and execute the internal runscript
- `sudo singularity build image.sif recipe` : build a singularity image from a recipe file

## Categories
containers, media, development

## Links
- More information: https://singularity-docs.readthedocs.io/en/latest/#commands
- tldr page: https://tldr.inbrowser.app/pages/common/singularity
