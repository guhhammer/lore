# scontrol

View information about and modify jobs.

## Usage
- `scontrol show job job_id` : show information for job
- `scontrol suspend job_id1,job_id2,...` : suspend a comma-separated list of running jobs
- `scontrol resume job_id1,job_id2,...` : resume a comma-separated list of suspended jobs
- `scontrol hold job_id1,job_id2,...` : hold a comma-separated list of queued jobs (Use `release` command to permit the jobs to be scheduled)
- `scontrol release job_id1,job_id2,...` : release a comma-separated list of suspended job

## Categories
processes

## Links
- More information: https://slurm.schedmd.com/scontrol.html
- tldr page: https://tldr.inbrowser.app/pages/linux/scontrol
