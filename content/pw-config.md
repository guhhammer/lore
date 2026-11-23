# pw-config

List configuration paths and sections that will be used by the PipeWire server and clients.

## Usage
- `pw-config` : list all configuration files that will be used
- `pw-config -n pipewire-pulse.conf` : list all configuration files that will be used by the PipeWire PulseAudio server
- `pw-config -n pipewire-pulse.conf list` : list all configuration sections used by the PipeWire PulseAudio server
- `pw-config -n jack.conf list context.properties` : list the `context.properties` fragments used by the JACK clients
- `pw-config -n jack.conf merge context.properties` : list the merged `context.properties` used by the JACK clients
- `pw-config -n pipewire.conf -r merge context.modules` : list the merged `context.modules` used by the PipeWire server and reformat
- `pw-config -h` : display help

## Links
- More information: https://docs.pipewire.org/page_man_pw-config_1.html
- tldr page: https://tldr.inbrowser.app/pages/linux/pw-config
