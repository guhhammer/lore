# aseqsend

Send arbitrary messages to selected ALSA MIDI seqencer port.

## Usage
- `aseqsend -p client:port -s file-name` : the general form, as the manual gives it
- `aseqsend -p client:port "hex encoded byte-string"` : another form of the same command
- `aseqsend -h` : prints a list of options
- `aseqsend -V` : prints the current version
- `aseqsend -l` : prints a list of possible output ports
- `aseqsend -v` : prints number of bytes actually sent

## Categories
network
