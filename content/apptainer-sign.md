# apptainer sign

Add digital signatures to a SIF container image.

## Usage
- `apptainer sign path/to/image.sif` : sign a container image using the default PGP key
- `apptainer sign --key path/to/private.pem path/to/image.sif` : sign a container image using a specific private key file
- `apptainer sign -k key_index path/to/image.sif` : sign a container image using a specific PGP key index
- `apptainer sign -g group_id path/to/image.sif` : sign a specific object group within the image
- `apptainer sign -i object_id path/to/image.sif` : sign a specific object by ID within the image
- `apptainer sign -h` : display help

## See also
- apptainer-verify

## Categories
containers, media, files

## Links
- More information: https://apptainer.org/docs/user/main/cli/apptainer_sign.html
- tldr page: https://tldr.inbrowser.app/pages/linux/apptainer-sign
