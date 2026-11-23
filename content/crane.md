# crane

Container images managing tool. Some subcommands such as `pull`, `push`, `copy`, etc. have their own usage documentation.

## Usage
- `crane auth login registry -u user -p password` : log in to a registry
- `crane catalog registry --full-ref` : list the repos in a registry
- `crane ls repository -o` : list the tags in a repository
- `crane pull image tarball` : pull remote images by reference and store their contents locally
- `crane push path/to/directory_or_tarball image` : push local image contents to a remote registry
- `crane tag image tag` : efficiently tag a remote image
- `crane cp source destination -a` : efficiently copy a remote image while retaining the digest value
- `crane delete image` : delete an image reference from its registry

## Categories
containers, media, files

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/crane
