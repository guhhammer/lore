# vagrant plugin

Manage Vagrant plugins.

## Usage
- `vagrant plugin list` : list all the plugins currently installed
- `vagrant plugin install vagrant_vbguest` : install a plugin from remote repositories, usually RubyGems
- `vagrant plugin install path/to/my_plugin.gem` : install a plugin from a local file source
- `vagrant plugin update` : update all installed plugins to their latest version
- `vagrant plugin update vagrant_vbguest` : update a plugin to the latest version
- `vagrant plugin uninstall vagrant_vbguest` : uninstall a specific plugin

## Categories
packages

## Links
- More information: https://developer.hashicorp.com/vagrant/docs/cli/plugin
- tldr page: https://tldr.inbrowser.app/pages/common/vagrant-plugin
- Wikipedia: https://en.wikipedia.org/wiki/Vagrant_(software)
