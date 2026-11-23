# pw-link

Manage links between ports in PipeWire.

## Usage
- `pw-link -oiI` : list all audio output and input ports with their IDs
- `pw-link output_port_name input_port_name` : create a link between an output and an input port
- `pw-link -d output_port_name input_port_name` : disconnect two ports
- `pw-link -lI` : list all links with their IDs
- `pw-link -h` : display help

## Categories
files, network

## Links
- More information: https://gitlab.freedesktop.org/pipewire/pipewire/-/wikis/Virtual-Devices
- tldr page: https://tldr.inbrowser.app/pages/linux/pw-link
