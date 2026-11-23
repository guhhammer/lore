# ippeveprinter

A simple IPP Everywhere printer server.

## Usage
- `ippeveprinter "service_name"` : run the server with a specific service name
- `ippeveprinter -P path/to/file.ppd "service_name"` : load printer attributes from a PPD file
- `ippeveprinter -c /usr/bin/file "service_name"` : run the `file` command whenever a job is sent to the server
- `ippeveprinter -d path/to/spool_directory "service_name"` : specify the directory that will hold the print files (by default, a directory under the user's temporary directory)
- `ippeveprinter -k "service_name"` : keep the print documents in the spool directory rather than deleting them
- `ippeveprinter -s speed "service_name"` : specify the printer speed in pages/minute unit (10 by default)

## Categories
files, hardware

## Links
- More information: https://openprinting.github.io/cups/doc/man-ippeveprinter.html
- tldr page: https://tldr.inbrowser.app/pages/common/ippeveprinter
