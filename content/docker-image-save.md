# docker image save

Export Docker images to archive.

## Usage
- `docker save image:tag >path/to/file.tar` : save an image by redirecting `stdout` to a `.tar` archive
- `docker save -o path/to/file.tar image:tag` : save an image to a `.tar` archive
- `docker save -o path/to/file.tar image_name` : save all tags of the image
- `docker save -o path/to/file.tar image_name:tag1 image_name:tag2 ...` : cherry-pick particular tags of an image to save

## Categories
containers, media, archives

## Links
- More information: https://docs.docker.com/reference/cli/docker/image/save/
- tldr page: https://tldr.inbrowser.app/pages/common/docker-image-save
