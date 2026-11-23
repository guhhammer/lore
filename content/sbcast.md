# sbcast

Send a file to a job's allocated nodes. This command should only be used from within a Slurm batch job.

## Usage
- `sbcast path/to/file path/to/destination` : send a file to all nodes allocated to the current job
- `sbcast --send-libs=yes path/to/executable path/to/destination` : autodetect shared libraries the transmitted file depends upon and transmit them as well

## Categories
files, processes

## Links
- More information: https://slurm.schedmd.com/sbcast.html
- tldr page: https://tldr.inbrowser.app/pages/linux/sbcast
