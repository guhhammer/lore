# lmms

Free, open source, cross-platform digital audio workstation. Render a `.mmp` or `.mmpz` project file, dump a `.mmpz` as XML, or start the GUI.

## Usage
- `lmms` : start the GUI
- `lmms -c path/to/config.xml` : start the GUI and load external config
- `lmms --import path/to/midi_or_hydrogen_file` : start the GUI and import MIDI or Hydrogen file
- `lmms --geometry x_sizexy_size+x_offset+y_offset` : start the GUI with a specified window size
- `lmms dump path/to/mmpz_file.mmpz` : dump a `.mmpz` file
- `lmms render path/to/mmpz_or_mmp_file` : render a project file
- `lmms rendertracks path/to/mmpz_or_mmp_file path/to/dump_directory` : render the individual tracks of a project file
- `lmms render -s 88200 -f ogg -l -o path/to/output_file.ogg` : render with custom samplerate, format, and as a loop

## See also
- mixxx

## Categories
files, development, text

## Links
- More information: https://manned.org/lmms
- tldr page: https://tldr.inbrowser.app/pages/common/lmms
- Wikipedia: https://en.wikipedia.org/wiki/LMMS
