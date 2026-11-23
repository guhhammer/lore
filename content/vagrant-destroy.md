# vagrant destroy

Stop a guest machine and destroy all its resources. Any boxes installed are kept intact.

## Usage
- `vagrant destroy` : destroy the currently running machine
- `vagrant destroy name|id` : target a machine by name or ID
- `vagrant destroy -f` : do not ask for confirmation before destroying
- `vagrant destroy -g` : shut down the machine gracefully

## Links
- More information: https://developer.hashicorp.com/vagrant/docs/cli/destroy
- tldr page: https://tldr.inbrowser.app/pages/common/vagrant-destroy
