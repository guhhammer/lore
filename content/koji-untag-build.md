# koji untag-build

Remove a tag from one or more builds.

## Usage
- `koji untag-build tag NVR1 NVR2 ...` : remove a tag from one or more builds
- `koji untag-build tag pkg1 pkg2 ... --all` : untag all versions of the package in this tag
- `koji untag-build tag pkg1 pkg2 ... --non-latest` : untag all versions of the package in this tag except the latest
- `koji untag-build tag NVR1 NVR2 ... -n` : test mode
- `koji untag-build tag NVR1 NVR2 ... -v` : print details
- `koji untag-build -h` : display help

## Categories
development

## Links
- More information: https://docs.pagure.org/koji/
- tldr page: https://tldr.inbrowser.app/pages/linux/koji-untag-build
