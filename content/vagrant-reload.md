# vagrant reload

Equivalent of running `halt` followed by `up`. A reload is usually required for changes in a Vagrantfile to take effect.

## Usage
- `vagrant reload` : reload the currently running machine
- `vagrant reload name|id` : target a machine by name or ID
- `vagrant reload --provision` : force the provisioners to run

## Links
- More information: https://developer.hashicorp.com/vagrant/docs/cli/reload
- tldr page: https://tldr.inbrowser.app/pages/common/vagrant-reload
