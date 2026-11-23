# gopass

Standard Unix Password Manager for Teams. Written in Go.

## Usage
- `gopass init` : initialize the configuration settings
- `gopass new` : create a new entry
- `gopass mounts` : show all stores
- `gopass mounts add store_name git_repo_url` : mount a shared Git store
- `gopass show keyword` : search interactively using a keyword
- `gopass find keyword` : search using a keyword
- `gopass sync` : sync all mounted stores
- `gopass store_name|path/to/directory|email@email.com` : show a particular password entry

## Categories
users, security

## Links
- More information: https://github.com/gopasspw/gopass/tree/master/docs/commands
- tldr page: https://tldr.inbrowser.app/pages/common/gopass
