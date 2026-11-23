# pw-midi2record

Record MIDI 2.0 clips through PipeWire. Note: This command is an alias of `pw-cat --record --midi-clip`.

## Usage
- `pw-midi2record path/to/file.mid` : record a MIDI 2.0 clip file from the default source
- `pw-midi2record -M midi|ump path/to/file.mid` : record a MIDI clip, forcing the MIDI format to use (defaults to `ump`)
- `pw-midi2record -h` : display help

## See also
- pw-cat
- pw-midi2play
- pw-midirecord

## Categories
media, files, shell

## Links
- More information: https://docs.pipewire.org/page_man_pw-cat_1.html
- tldr page: https://tldr.inbrowser.app/pages/linux/pw-midi2record
