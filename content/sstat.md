# sstat

View information about running jobs.

## Usage
- `sstat -j job_id` : display status information of a comma-separated list of jobs
- `sstat -p -j job_id -o JobID,AveCPU,AveVMSize` : display job ID, average CPU, and average virtual memory size of a comma-separated list of jobs, with pipes as column delimiters
- `sstat -e` : display list of fields available

## Categories
processes, desktop

## Links
- More information: https://slurm.schedmd.com/sstat.html
- tldr page: https://tldr.inbrowser.app/pages/linux/sstat
