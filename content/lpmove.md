# lpmove

Move a job or all jobs to another printer.

## Usage
- `lpmove job_id new_printer` : move a specific job to `new_printer`
- `lpmove old_printer-job_id new_printer` : move a job from `old_printer` to `new_printer`
- `lpmove old_printer new_printer` : move all jobs from `old_printer` to `new_printer`
- `lpmove -h server job_id new_printer` : move a specific job to `new_printer` on a specific server

## See also
- cancel
- lp
- lpr
- lprm

## Categories
processes, files, hardware

## Links
- More information: https://openprinting.github.io/cups/doc/man-lpmove.html
- tldr page: https://tldr.inbrowser.app/pages/common/lpmove
