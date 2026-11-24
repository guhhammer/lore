# apptainer verify

Verify digital signatures of SIF container images.

## Usage
- `apptainer verify path/to/image.sif` : verify a container image using the default PGP keyring
- `apptainer verify --key path/to/public.pem path/to/image.sif` : verify a container image using a specific public key file
- `apptainer verify --certificate path/to/certificate.pem path/to/image.sif` : verify a container image using a certificate file
- `apptainer verify -a path/to/image.sif` : verify all objects in the image
- `apptainer verify -g group_id path/to/image.sif` : verify a specific object group within the image
- `apptainer verify -i object_id path/to/image.sif` : verify a specific object by ID within the image
- `apptainer verify -j path/to/image.sif` : output verification results in JSON format
- `apptainer verify -h` : display help

## See also
- apptainer-sign

## Categories
containers, media, files

## Links
- More information: https://apptainer.org/docs/user/main/cli/apptainer_verify.html
- tldr page: https://tldr.inbrowser.app/pages/linux/apptainer-verify
- Wikipedia: https://en.wikipedia.org/wiki/Apptainer
