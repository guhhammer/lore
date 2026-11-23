# keyd

Remap keys.

## Usage
- `systemctl enable keyd --now` : start and enable the `keyd` service
- `sudo keyd -m` : display keypress information
- `sudo keyd reload` : reset bindings and reload the configuration files in `/etc/keyd`
- `keyd list-keys` : list all valid key names
- `keyd check` : check the detected configuration files for errors
- `sudo keyd bind "pressed_key = output_key"` : create a temporary binding

## Categories
users

## Links
- More information: https://manned.org/keyd
- tldr page: https://tldr.inbrowser.app/pages/linux/keyd
