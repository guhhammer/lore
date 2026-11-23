# pw-metadata

Monitor, set, and delete metadata on PipeWire objects.

## Usage
- `pw-metadata` : show metadata in `default` name
- `pw-metadata -n settings 0` : show metadata with ID 0 in `settings`
- `pw-metadata -l` : list all available metadata objects
- `pw-metadata -m` : keep running and log the changes to the metadata
- `pw-metadata -d` : delete all metadata
- `pw-metadata -n settings 0 log.level 1` : set `log.level` to 1 in `settings`
- `pw-metadata -h` : display help

## See also
- pipewire
- pw-mon
- pw-cli

## Categories
monitoring

## Links
- More information: https://docs.pipewire.org/page_man_pw-metadata_1.html
- tldr page: https://tldr.inbrowser.app/pages/linux/pw-metadata
