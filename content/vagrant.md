# vagrant

Manage lightweight, reproducible, and portable development environments. Some subcommands such as `box`, `snapshot`, `halt`, etc. have their own usage documentation.

## Usage
- `vagrant init` : create a `Vagrantfile` in the current directory with the base Vagrant box
- `vagrant init ubuntu/focal64` : create a `Vagrantfile` with a box from the Vagrant Public Registry
- `vagrant up` : start and provision the Vagrant environment
- `vagrant suspend` : suspend the machine
- `vagrant halt` : halt the machine
- `vagrant ssh` : connect to the machine via SSH
- `vagrant ssh-config` : output the SSH configuration file of the running Vagrant machine
- `vagrant box list` : list all local boxes

## Categories
network, documents, monitoring

## Links
- More information: https://developer.hashicorp.com/vagrant/docs/cli
- tldr page: https://tldr.inbrowser.app/pages/common/vagrant
