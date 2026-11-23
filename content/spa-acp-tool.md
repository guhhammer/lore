# spa-acp-tool

Debug ALSA card profile probing without running PipeWire.

## Usage
- `spa-acp-tool l` : list all ALSA objects
- `spa-acp-tool c card_id` : probe a specific ALSA card by ID
- `spa-acp-tool lpr card_id` : list available ALSA profiles for a card
- `spa-acp-tool spr profile_id` : set the active ALSA profile by ID
- `spa-acp-tool ld card_id` : list available devices for a card
- `spa-acp-tool gv device_id` : get volume from a device
- `spa-acp-tool v device_id volume_level` : set volume on a device
- `spa-acp-tool m device_id` : toggle mute state on a device

## Categories
hardware, development

## Links
- More information: https://docs.pipewire.org/page_man_spa-acp-tool_1.html
- tldr page: https://tldr.inbrowser.app/pages/linux/spa-acp-tool
