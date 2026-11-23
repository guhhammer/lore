# apptainer delete

Delete container images from a remote library.

## Usage
- `apptainer delete library://user/collection/container:tag` : delete an image from the Container Library
- `apptainer delete -A amd64|arm64|ppc64le library://user/collection/container:tag` : delete an image for a specific architecture
- `apptainer delete -F library://user/collection/container:tag` : force delete an image without confirmation
- `apptainer delete --library https://library.example.com library://user/collection/container:tag` : delete an image from a specific library server
- `apptainer delete --no-https library://hostname/user/collection/container:tag` : delete an image using HTTP instead of HTTPS
- `apptainer delete -h` : display help

## Categories
containers, development, media

## Links
- More information: https://apptainer.org/docs/user/main/cli/apptainer_delete.html
- tldr page: https://tldr.inbrowser.app/pages/linux/apptainer-delete
