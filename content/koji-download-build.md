# koji download-build

Download a built package.

## Usage
- `koji download-build BuildID|RPM|NVR` : download all RPMs from a specific build
- `koji download-build BuildID|RPM|NVR --key key` : download RPMs signed with the given key
- `koji download-build BuildID|RPM|NVR --arch x86_64,aarch64,noarch,...` : only download RPMs for given arch
- `koji download-build RPM --rpm` : download the given RPM
- `koji download-build -h` : display help

## Categories
network, development, packages

## Links
- More information: https://docs.pagure.org/koji/
- tldr page: https://tldr.inbrowser.app/pages/linux/koji-download-build
