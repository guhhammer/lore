# mt

Control magnetic tape drive operation (commonly LTO tape).

## Usage
- `mt -f /dev/nstX status` : check the status of a tape drive
- `mt -f /dev/nstX rewind` : rewind the tape to beginning
- `mt -f /dev/nstX fsf count` : move forward a given files, then position the tape on first block of next file
- `mt -f /dev/nstX asf count` : rewind the tape, then position the tape at beginning of the given file
- `mt -f /dev/nstX eod` : position the tape at the end of valid data
- `mt -f /dev/nstX eject` : rewind the tape and unload/eject it
- `mt -f /dev/nstX eof` : write EOF (End-of-file) mark at the current position

## Categories
disks, files

## Links
- More information: https://manned.org/mt
- tldr page: https://tldr.inbrowser.app/pages/linux/mt
