# slurmd

Monitor all tasks running on the compute node, accept tasks, launch tasks, and kill running tasks upon request.

## Usage
- `slurmd -b` : report node rebooted when daemon restarted (Used for testing purposes)
- `slurmd -N nodename` : run the daemon with the given nodename
- `slurmd -L path/to/output_file` : write log messages to the specified file
- `slurmd -f path/to/file` : read configuration from the specified file
- `slurmd -h` : display help

## Categories
files, monitoring

## Links
- More information: https://slurm.schedmd.com/slurmd.html
- tldr page: https://tldr.inbrowser.app/pages/linux/slurmd
