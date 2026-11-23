# vagrant box

Manage Vagrant boxes (virtual machine images).

## Usage
- `vagrant box list` : list all installed boxes
- `vagrant box add hashicorp/bionic64` : add a new box
- `vagrant box add my-box https://example.com/my-box.box` : add a box from a custom URL
- `vagrant box remove hashicorp/bionic64` : remove an installed box
- `vagrant box update` : update all boxes that are in use in the current Vagrant environment
- `vagrant box update --box bento/debian-12` : update a specific box
- `vagrant box outdated` : check if there is a new version available for the box that you are using
- `vagrant box prune` : clean up old versions of installed boxes

## Categories
network, containers, media

## Links
- More information: https://developer.hashicorp.com/vagrant/docs/cli/box
- tldr page: https://tldr.inbrowser.app/pages/common/vagrant-box
