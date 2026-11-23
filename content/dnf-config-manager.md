# dnf config-manager

Manage DNF configuration options and repositories on Fedora-based systems. Not default to `dnf` but supported via `dnf-plugins-core`.

## Usage
- `dnf config-manager --add-repo=repository_url` : add (and enable) a repository from a URL
- `dnf config-manager --dump` : print current configuration values
- `dnf config-manager --enable repository_id` : enable a specific repository
- `dnf config-manager --disable repository_id1 repository_id2 ...` : disable specified repositories
- `dnf config-manager --setopt=option=value` : set a configuration option for a repository
- `dnf config-manager --help-cmd` : display help

## Categories
network, packages

## Links
- More information: https://dnf-plugins-core.readthedocs.io/en/latest/config_manager.html
- tldr page: https://tldr.inbrowser.app/pages/linux/dnf-config-manager
