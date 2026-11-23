# a2ping

Convert images into EPS or PDF files.

## Usage
- `a2ping path/to/image.ext path/to/output.pdf` : convert an image to PDF (Note: Specifying an output filename is optional)
- `a2ping --nocompress none|zip|best|flate path/to/file` : compress the document using the specified method
- `a2ping --nohires path/to/file` : scan HiResBoundingBox if present (defaults to yes)
- `a2ping --below path/to/file` : allow page content below and left of the origin (defaults to no)
- `a2ping --gsextra arguments path/to/file` : pass extra arguments to `gs`
- `a2ping --extra arguments path/to/file` : pass extra arguments to external program (i.e. `pdftops`)
- `a2ping -h` : display help

## Categories
files, media, documents

## Links
- More information: https://manned.org/a2ping
- tldr page: https://tldr.inbrowser.app/pages/common/a2ping
