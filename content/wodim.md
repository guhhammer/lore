# wodim

This command is an alias of `cdrecord`.

## Usage
- `cdrecord --devices` : display optical drives available to `cdrecord`
- `cdrecord dev=/dev/optical_drive -audio track*.cdaudio` : record ("burn") an audio-only disc
- `cdrecord -eject dev=/dev/optical_drive -data file.iso` : burn a file to a disc, ejecting the disc once done (some recorders require this)
- `cdrecord -tao dev=/dev/optical_drive -data file.iso` : burn a file to the disc in an optical drive, potentially writing to multiple discs in succession

## Categories
files, media, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/wodim
