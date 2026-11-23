# secret-tool

Store and retrieve passwords, part of the `libsecret` package. Communicates with Freedesktop secret service implementations such as `gnome-keyring`.

## Usage
- `secret-tool store --label=label key value` : store a secret with an optional label
- `secret-tool lookup key key` : retrieve a secret
- `secret-tool search key key` : get more information about a secret
- `secret-tool clear key key` : delete a stored secret

## Categories
desktop, packages, system

## Links
- More information: https://gnome.pages.gitlab.gnome.org/libsecret/
- tldr page: https://tldr.inbrowser.app/pages/linux/secret-tool
