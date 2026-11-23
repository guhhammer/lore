# cups

Open source printing system. Note: CUPS isn't a command, but a set of commands.

## Usage
- `cupsd` : start `cupsd` in the background, aka. as a daemon
- `cupsd -f` : start `cupsd` on the foreground
- `lpadmin -d printer` : set the default printer
- `lpadmin -x printer|class` : delete a specific printer or class
- `echo "test" | lp` : print the output of a command to the default printer (see `lpstat` command)
- `lp path/to/file` : print a file to the default printer
- `lpstat` : list all queued print jobs
- `lpstat -p` : list printers present on the machine and whether they are enabled for printing

## Categories
hardware, documents, files

## Links
- More information: https://www.cups.org/index.html
- tldr page: https://tldr.inbrowser.app/pages/common/cups
