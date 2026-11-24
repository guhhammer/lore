# vagrant halt

Shut down the running machine Vagrant is managing.

## Usage
- `vagrant halt` : halt the currently running Vagrant machine gracefully
- `vagrant halt id_or_name` : halt a specific machine by its ID or name gracefully
- `vagrant halt -f` : forcefully halt the current running machine(s) (This can affect multiple running machines if they are part of the same Vagrant environment)
- `vagrant halt -f id_or_name` : forcefully halt a specific machine by its ID or name

## See also
- vagrant box
- vagrant plugin
- vagrant validate

## Links
- More information: https://developer.hashicorp.com/vagrant/docs/cli/halt
- tldr page: https://tldr.inbrowser.app/pages/common/vagrant-halt
- Wikipedia: https://en.wikipedia.org/wiki/Vagrant_(software)
