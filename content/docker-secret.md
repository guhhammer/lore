# docker secret

Manage Docker swarm secrets.

## Usage
- `command | docker secret create secret_name -` : create a new secret from `stdin`
- `docker secret create secret_name path/to/file` : create a new secret from a file
- `docker secret ls` : list all secrets
- `docker secret inspect --pretty secret_name1 secret_name2 ...` : display detailed information on one or multiple secrets in a human friendly format
- `docker secret rm secret_name1 secret_name2 ...` : remove one or more secrets

## Categories
containers, files

## Links
- More information: https://docs.docker.com/reference/cli/docker/secret/
- tldr page: https://tldr.inbrowser.app/pages/common/docker-secret
- Wikipedia: https://en.wikipedia.org/wiki/Docker_(software)
