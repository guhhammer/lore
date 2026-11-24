# git credential

Retrieve and store user credentials.

## Usage
- `echo "url=http://example.com" | git credential fill` : display credential information, retrieving the username and password from configuration files
- `echo "url=http://example.com" | git credential approve` : send credential information to all configured credential helpers to store for later use
- `echo "url=http://example.com" | git credential reject` : erase the specified credential information from all the configured credential helpers

## Categories
version-control, network, users

## Links
- More information: https://git-scm.com/docs/git-credential
- tldr page: https://tldr.inbrowser.app/pages/common/git-credential
- Wikipedia: https://en.wikipedia.org/wiki/Git
