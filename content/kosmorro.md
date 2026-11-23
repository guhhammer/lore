# kosmorro

Compute the ephemerides and the events for a date at a position on Earth.

## Usage
- `kosmorro -p "48.7996,2.3511"` : get ephemerides for Paris, France
- `kosmorro -p "48.7996,2.3511" -t "Europe/Paris"` : get ephemerides for Paris, France, on its timezone
- `kosmorro -p "48.7996,2.3511" -d "2020-06-09"` : get ephemerides for Paris, France, on June 9th, 2020
- `kosmorro -p "48.7996,2.3511" -d "+2d"` : get ephemerides for Paris, France, in two days
- `kosmorro -o "path/to/file.pdf"` : generate a PDF (Note: TeXLive must be installed)

## Links
- More information: https://kosmorro.space/cli/manpage/
- tldr page: https://tldr.inbrowser.app/pages/common/kosmorro
