# krunvm

Create MicroVMs from OCI images.

## Usage
- `krunvm create docker.io/fedora --cpus number_of_vcpus --mem memory_in_megabytes --name "name"` : create MicroVM based on Fedora
- `krunvm start "image_name"` : start a specific image
- `krunvm list` : list images
- `krunvm changevm --cpus number_of_vcpus --mem memory_in_megabytes --name "new_vm_name" "current_vm_name"` : change a specific image
- `krunvm delete "image_name"` : delete a specific image

## Categories
containers, media

## Links
- More information: https://github.com/containers/krunvm/blob/main/docs/krunvm.1.txt
- tldr page: https://tldr.inbrowser.app/pages/common/krunvm
