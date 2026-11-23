# systemd-id128

Generate and print sd-128 identifiers.

## Usage
- `systemd-id128 new` : generate a new random identifier
- `systemd-id128 machine-id` : print the identifier of the current machine
- `systemd-id128 boot-id` : print the identifier of the current boot
- `systemd-id128 invocation-id` : print the identifier of the current service invocation (this is available in systemd services)
- `systemd-id128 new -u` : generate a new random identifier and print it as a UUID (five groups of digits separated by hyphens)

## Categories
system, documents

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/systemd-id128.html
- tldr page: https://tldr.inbrowser.app/pages/linux/systemd-id128
