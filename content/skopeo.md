# skopeo

Container image management toolbox. Provides various utility commands to manage remote container images.

## Usage
- `skopeo inspect docker://registry_hostname/image:tag` : inspect a remote image from a registry
- `skopeo list-tags docker://registry_hostname/image` : list available tags for a remote image
- `skopeo copy docker://registry_hostname/image:tag dir:path/to/directory` : download an image from a registry
- `skopeo copy docker://source_registry/image:tag docker://destination_registry/image:tag` : copy an image from one registry to another
- `skopeo delete docker://registry_hostname/image:tag` : delete an image from a registry
- `skopeo login --username username registry_hostname` : log in to a registry

## Categories
containers, media, files

## Links
- More information: https://github.com/containers/skopeo/blob/main/docs/skopeo.1.md
- tldr page: https://tldr.inbrowser.app/pages/common/skopeo
