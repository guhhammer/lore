# cdrecord

Record data to CDs or DVDs. Some invocations of cdrecord can cause destructive actions, such as erasing all the data on a disc.

## Usage
- `cdrecord --devices` : display optical drives available to `cdrecord`
- `cdrecord dev=/dev/optical_drive -audio track*.cdaudio` : record ("burn") an audio-only disc
- `cdrecord -eject dev=/dev/optical_drive -data file.iso` : burn a file to a disc, ejecting the disc once done (some recorders require this)
- `cdrecord -tao dev=/dev/optical_drive -data file.iso` : burn a file to the disc in an optical drive, potentially writing to multiple discs in succession

## Categories
media, files

## Links
- More information: https://manned.org/cdrecord
- tldr page: https://tldr.inbrowser.app/pages/linux/cdrecord
- Wikipedia: https://en.wikipedia.org/wiki/Cdrtools
