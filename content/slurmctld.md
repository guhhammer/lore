# slurmctld

Monitor all other Slurm daemons and resources, accept work (jobs), and allocate resources to those jobs.

## Usage
- `slurmctld -c` : clear all previous `slurmctld` states from its last checkpoint
- `slurmctld -n value` : set the daemon's nice value to the specified value, typically a negative number
- `slurmctld -L path/to/output_file` : write log messages to the specified file
- `slurmctld -h` : display help
- `slurmctld -V` : display version

## Categories
processes, monitoring

## Links
- More information: https://slurm.schedmd.com/slurmctld.html
- tldr page: https://tldr.inbrowser.app/pages/linux/slurmctld
