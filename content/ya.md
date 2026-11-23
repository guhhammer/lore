# ya

Manage Yazi plugins and flavors, publish or subscribe to DDS messages.

## Usage
- `ya pkg add package` : add a package
- `ya pkg delete package` : delete a package
- `ya pkg upgrade` : upgrade all packages
- `ya pkg list` : list all packages
- `ya emit command arguments` : emit a command to be executed by the current instance
- `ya emit-to receiver command arguments` : emit commands to a specific remote instance
- `ya pub --str string_message` : publish a message to the current instance with string body
- `ya sub message_kind` : subscribe to a specific kind of message from a remote instance

## Categories
packages

## Links
- More information: https://yazi-rs.github.io/docs/cli/
- tldr page: https://tldr.inbrowser.app/pages/common/ya
