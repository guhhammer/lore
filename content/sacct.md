# sacct

Display accounting data from the Slurm service.

## Usage
- `sacct` : display job ID, job name, partition, account, number of allocated cpus, job state, and job exit codes for recent jobs
- `sacct -b` : display job ID, job state, job exit code for recent jobs
- `sacct -j job_id -X` : display the allocations of a job
- `sacct -j job_id -o Elapsed,JobName,ReqCPUS,ReqMem` : display elapsed time, job name, number of requested CPUs, and memory requested of a job
- `sacct -S $(date -d "1 week ago" +'%F')` : display recent jobs that occurred from one week ago up to the present day
- `sacct -o JobID,JobName%100` : output a larger number of characters for an attribute

## Categories
processes, desktop, system

## Links
- More information: https://slurm.schedmd.com/sacct.html
- tldr page: https://tldr.inbrowser.app/pages/linux/sacct
