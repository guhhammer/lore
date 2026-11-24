# apptainer registry

Manage authentication to OCI/Docker registries.

## Usage
- `apptainer registry list` : list all configured registry credentials
- `apptainer registry login -u username docker://registry` : log in to a registry with a username (password will be prompted)
- `apptainer registry login -u username oras://registry` : log in to a custom OCI registry
- `apptainer registry login -u username -p password docker://registry` : log in with username and password
- `echo "password" | apptainer registry login -u username --password-stdin docker://registry` : log in with a password from `stdin`
- `apptainer registry login --authfile path/to/auth.json -u username docker://registry` : log in using a custom authentication file
- `apptainer registry logout docker://registry` : log out from a registry
- `apptainer registry logout --authfile path/to/auth.json docker://registry` : log out using a custom authentication file

## See also
- apptainer pull
- apptainer push

## Categories
users, containers, monitoring

## Links
- More information: https://apptainer.org/docs/user/main/cli/apptainer_registry.html
- tldr page: https://tldr.inbrowser.app/pages/linux/apptainer-registry
- Wikipedia: https://en.wikipedia.org/wiki/Apptainer
