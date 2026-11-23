# immich

Manage Immich servers.

## Usage
- `immich login server_url/api server_key` : authenticate to Immich server
- `immich upload file1.jpg file2.jpg ...` : upload some image files
- `immich upload -r path/to/directory` : upload a directory including subdirectories
- `immich upload -r path/to/directory -A "My summer holiday"` : create an album based on a directory
- `immich upload -r path/to/directory -i **/Raw/** **/*.tif` : skip assets matching a `glob` pattern
- `immich upload -r path/to/directory -H` : include hidden files

## See also
- immich-go

## Categories
files, network

## Links
- More information: https://docs.immich.app/features/command-line-interface/
- tldr page: https://tldr.inbrowser.app/pages/common/immich
