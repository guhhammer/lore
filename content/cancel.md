# cancel

Cancel print jobs.

## Usage
- `cancel` : cancel the current job of the default printer (set with `lpoptions -d printer_name`)
- `cancel -u username` : cancel the jobs of the default printer owned by a specific user
- `cancel printer` : cancel the current job of a specific printer
- `cancel printer-job_id` : cancel a specific job from a specific printer
- `cancel -a` : cancel all jobs of all printers
- `cancel -a printer` : cancel all jobs of a specific printer
- `cancel -h server -x` : cancel the current job of a specific host server and then delete (x) job data files

## See also
- lp
- lpmove
- lpstat

## Categories
processes, hardware, documents

## Links
- More information: https://openprinting.github.io/cups/doc/man-cancel.html
- tldr page: https://tldr.inbrowser.app/pages/common/cancel
