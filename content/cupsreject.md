# cupsreject

Reject jobs sent to printers. Note: Destination is referred as a printer or a class of printers.

## Usage
- `cupsreject destination1 destination2 ...` : reject print jobs to the specified destinations
- `cupsreject -h server destination1 destination2 ...` : specify a different server
- `cupsreject -r reason destination1 destination2 ...` : specify a reason string ("Reason Unknown" by default)

## See also
- cupsaccept
- cupsenable
- cupsdisable
- lpstat

## Categories
processes, hardware

## Links
- More information: https://www.cups.org/doc/man-cupsaccept.html
- tldr page: https://tldr.inbrowser.app/pages/common/cupsreject
