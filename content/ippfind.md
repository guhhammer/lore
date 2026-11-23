# ippfind

Find services registered with a DNS server or available through local devices.

## Usage
- `ippfind -l` : list IPP printers registered on the network with their status
- `ippfind --txt-pdl application/postscript -x ipptool -f path/to/document.ps '{}' print-job.test \;` : send a specific PostScript document to every PostScript printer on the network
- `ippfind --txt-pdl application/postscript -x ipptool -f onepage-letter.ps '{}' print-job.test \;` : send a PostScript test document to every PostScript printer on the network
- `ippfind --txt-pdl application/postscript -h regex -x ipptool -f onepage-letter.ps '{}' print-job.test \;` : send a PostScript test document to every PostScript printer on the network, whose name matches a `regex`

## See also
- ipptool
- ippeveprinter

## Categories
network, documents, hardware

## Links
- More information: https://openprinting.github.io/cups/doc/man-ippfind.html
- tldr page: https://tldr.inbrowser.app/pages/common/ippfind
