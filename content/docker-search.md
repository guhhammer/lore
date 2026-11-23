# docker search

Search for Docker images on Docker Hub.

## Usage
- `docker search keyword` : search for Docker images by name or keyword
- `docker search -f is-official=true keyword` : search for images and only show official ones
- `docker search -f is-automated=true keyword` : search for images and only show automated builds
- `docker search -f stars=number keyword` : search for images with a minimum number of stars
- `docker search --limit number keyword` : limit the number of results
- `docker search -f "\{\{.Name\}\}: \{\{.Description\}\}" keyword` : customize the output format

## Categories
containers, media

## Links
- More information: https://docs.docker.com/reference/cli/docker/search/
- tldr page: https://tldr.inbrowser.app/pages/common/docker-search
