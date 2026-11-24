# puppet

Manage and automate the configuration of servers. Puppet uses a declarative language to define system configurations and apply them automatically. Some subcommands such as `agent`, `apply` have their own usage documentation.

## Usage
- `puppet apply path/to/file.pp` : apply a Puppet manifest file to configure the system
- `puppet apply --noop path/to/file.pp` : simulate applying a manifest to preview changes
- `puppet parser validate path/to/file.pp` : validate the syntax of a Puppet manifest
- `puppet agent -t` : run the Puppet agent to fetch and apply configurations from the master
- `puppet help subcommand` : display help for a specific subcommand
- `puppet -h` : display general help
- `puppet -V` : display version

## Categories
files, desktop, documents

## Links
- More information: https://github.com/puppetlabs/puppet/blob/main/references/man/overview.md
- tldr page: https://tldr.inbrowser.app/pages/common/puppet
- Wikipedia: https://en.wikipedia.org/wiki/Puppet_(software)
