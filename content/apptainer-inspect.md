# apptainer inspect

Display metadata of Apptainer container images.

## Usage
- `apptainer inspect path/to/image.sif` : show the labels of an image (default)
- `apptainer inspect -d path/to/image.sif` : show the definition file used to build the image
- `apptainer inspect -r path/to/image.sif` : show the runscript for the image
- `apptainer inspect -e path/to/image.sif` : show the environment variables of the image
- `apptainer inspect --list-apps path/to/image.sif` : list all apps in the container
- `apptainer inspect --all path/to/image.sif` : show all available data in JSON format
- `apptainer inspect -h` : display help

## Categories
containers, media, files

## Links
- More information: https://apptainer.org/docs/user/main/cli/apptainer_inspect.html
- tldr page: https://tldr.inbrowser.app/pages/linux/apptainer-inspect
- Wikipedia: https://en.wikipedia.org/wiki/Apptainer
