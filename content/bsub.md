# bsub

Submit batch jobs to LSF (Load Sharing Facility) scheduler.

## Usage
- `bsub path/to/script.sh` : submit a script file as a job
- `bsub -q queue_name make all` : submit a job to a specific queue
- `bsub -J job_name --output path/to/output.log --error path/to/error.log path/to/script.sh` : submit a job with a name and redirect output and error
- `bsub -n 8 -M 16G cargo build --release` : request 8 CPU cores and 16GB memory for a command
- `bsub -I bash` : run an interactive shell in the current session
- `bsub -W 45 path/to/script.sh` : submit a job with a runtime limit of 45 minutes

## Categories
processes, files, monitoring

## Links
- More information: https://www.ibm.com/docs/spectrum-lsf/latest?topic=reference-bsub
- tldr page: https://tldr.inbrowser.app/pages/linux/bsub
