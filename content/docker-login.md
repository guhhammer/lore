# docker login

Log into a Docker registry.

## Usage
- `docker login` : interactively log into a registry
- `docker login -u username` : log into a registry with a specific username (user will be prompted for a password)
- `docker login -u username -p password server` : log into a registry with username and password
- `echo "password" | docker login -u username --password-stdin` : log into a registry with password from `stdin`

## Categories
users, containers, monitoring

## Links
- More information: https://docs.docker.com/reference/cli/docker/login/
- tldr page: https://tldr.inbrowser.app/pages/common/docker-login
- Wikipedia: https://en.wikipedia.org/wiki/Docker_(software)
