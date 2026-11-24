# podman save

Save an image to a local file or directory.

## Usage
- `podman save -o path/to/file.tar image:tag` : save an image to a `.tar` file
- `podman save image:tag >path/to/file.tar` : save an image to `stdout`
- `podman save image:tag | gzip|bzip2|xz|zstd|zstdchunked >path/to/file.tar.ext` : save an image with compression
- `podman save image:tag | zstd -T 0 --ultra | pv | ssh username@remote_host podman load` : transfer an image to remote system with on-the-fly compression and progress bar

## See also
- podman load
- podman export

## Categories
containers, files, media

## Links
- More information: https://docs.podman.io/en/latest/markdown/podman-save.1.html
- tldr page: https://tldr.inbrowser.app/pages/common/podman-save
- Wikipedia: https://en.wikipedia.org/wiki/Podman
