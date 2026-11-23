# ipptool

Issue IPP requests and receive printer's/server's responses.

## Usage
- `ipptool ipp://printer_uri get-printer-attributes.test` : get all attributes and their values supported by a printer
- `ipptool ipp://printer_uri get-completed-jobs.test` : get the list of completed jobs of a printer
- `ipptool -d recipient=mailto:email ipp://printer_uri create-printer-subscription.test` : send an email notification when a printer changes

## See also
- ippfind
- ippeveprinter

## Categories
hardware, development

## Links
- More information: https://openprinting.github.io/cups/doc/man-ipptool.html
- tldr page: https://tldr.inbrowser.app/pages/common/ipptool
