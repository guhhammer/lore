# pw-midi2play

Play MIDI 2.0 clips through PipeWire. Note: This command is an alias of `pw-cat --playback --midi-clip`.

## Usage
- `pw-midi2play path/to/file.mid` : play a MIDI 2.0 clip file over the default target
- `pw-midi2play -M midi|ump path/to/file.mid` : play a MIDI clip, forcing the MIDI format to use (defaults to `ump`)
- `pw-midi2play -h` : display help

## See also
- pw-cat
- pw-midi2record
- pw-midiplay

## Categories
files, shell

## Links
- More information: https://docs.pipewire.org/page_man_pw-cat_1.html
- tldr page: https://tldr.inbrowser.app/pages/linux/pw-midi2play
