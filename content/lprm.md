# lprm

Cancel queued print jobs of a server.

## Usage
- `lprm` : cancel current job on the default printer
- `lprm -h server job_id` : cancel a job of a specific server
- `lprm -h server:port job_id` : cancel a job of a specific server in a specific port
- `lprm -E job_id1 job_id2 ...` : cancel multiple jobs with an encrypted connection to the server
- `lprm -` : cancel all jobs
- `lprm -P destination/instance` : cancel the current job of a specific printer or class

## See also
- lpq

## Categories
processes, network, documents

## Links
- More information: https://openprinting.github.io/cups/doc/man-lprm.html
- tldr page: https://tldr.inbrowser.app/pages/common/lprm
