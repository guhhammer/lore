# xorrecord

Emulation of CD/DVD/BD program cdrecord by program xorriso.

## Usage
- `xorrecord [ options ] dev=device [track_source]` : the general form, as the manual gives it
- `xorrecord --devices` : print the list of accessible CD, DVD, or BD drives to standard output
- `xorrecord -toc` : print  a table of content of the medium in the drive
- `xorrecord -isosize` : try to obtain the track size from the content of the track source
- `xorrecord -pad` : the  same  as  padsize=15s
