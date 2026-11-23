# lp

Print files.

## Usage
- `echo "test" | lp` : print the output of a command to the default printer (see `lpstat` command)
- `lp path/to/file` : print a file to the default printer
- `lp -d printer_name path/to/file` : print a file to a named printer (see `lpstat` command)
- `lp -n n path/to/file` : print `n` copies of a file to the default printer
- `lp -P 1,3-5,16 path/to/file` : print only certain pages to the default printer (print pages 1, 3-5, and 16)
- `lp -i job_id -H resume` : resume printing a job

## See also
- lpstat
- lpoptions

## Categories
files, documents, hardware

## Links
- More information: https://manned.org/lp
- tldr page: https://tldr.inbrowser.app/pages/common/lp
