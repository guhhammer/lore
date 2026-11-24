# vagrant init

Initialize a Vagrant environment in the current directory by creating a `Vagrantfile`.

## Usage
- `vagrant init` : create a `Vagrantfile`
- `vagrant init -m` : create a `Vagrantfile` without instructional comments
- `vagrant init box_name box_url` : specify the box name and URL
- `vagrant init --box-version version box_name` : create a `Vagrantfile` with a specific box version
- `vagrant init -o -` : send the `Vagrantfile` to `stdout`
- `vagrant init -f` : overwrite an existing `Vagrantfile`
- `vagrant init --template path/to/file.erb` : provide a custom ERB template for generating the `Vagrantfile`

## Categories
files, network

## Links
- More information: https://developer.hashicorp.com/vagrant/docs/cli/init
- tldr page: https://tldr.inbrowser.app/pages/common/vagrant-init
- Wikipedia: https://en.wikipedia.org/wiki/Vagrant_(software)
