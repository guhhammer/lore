# vagrant snapshot

Manage snapshots of Vagrant machines.

## Usage
- `vagrant snapshot save snapshot_name` : take a snapshot of the machine (running or stopped)
- `vagrant snapshot restore snapshot_name` : restore a snapshot and start the machine
- `vagrant snapshot restore --no-start snapshot_name` : restore a snapshot without starting the machine
- `vagrant snapshot delete snapshot_name` : delete a snapshot
- `vagrant snapshot list` : list available snapshots of the machine

## Links
- More information: https://developer.hashicorp.com/vagrant/docs/cli/snapshot
- tldr page: https://tldr.inbrowser.app/pages/common/vagrant-snapshot
- Wikipedia: https://en.wikipedia.org/wiki/Vagrant_(software)
