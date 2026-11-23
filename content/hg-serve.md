# hg serve

Start a standalone Mercurial web server for browsing repositories.

## Usage
- `hg serve` : start a web server instance
- `hg serve -p port` : start a web server instance on the specified port
- `hg serve -a address` : start a web server instance on the specified listening address
- `hg serve -n name` : start a web server instance with a specific identifier
- `hg serve --style style` : start a web server instance using the specified theme (see the templates directory)
- `hg serve --certificate path/to/certificate` : start a web server instance using the specified SSL certificate bundle

## Categories
network, security, version-control

## Links
- More information: https://www.mercurial-scm.org/help/commands/serve
- tldr page: https://tldr.inbrowser.app/pages/common/hg-serve
