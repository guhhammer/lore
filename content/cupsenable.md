# cupsenable

Start printers and classes. Note: Destination is referred as a printer or a class of printers.

## Usage
- `cupsenable destination1 destination2 ...` : start one or more destination(s)
- `cupsenable --release destination` : resume printing of pending jobs of a destination (use after `cupsdisable` with `--hold`)
- `cupsenable -c destination1 destination2 ...` : cancel all jobs of the specified destination(s)

## See also
- cupsdisable
- cupsaccept
- cupsreject
- lpstat

## Categories
hardware

## Links
- More information: https://www.cups.org/doc/man-cupsenable.html
- tldr page: https://tldr.inbrowser.app/pages/common/cupsenable
