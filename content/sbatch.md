# sbatch

Submit a batch job to the SLURM scheduler.

## Usage
- `sbatch path/to/job.sh` : submit a batch job
- `sbatch --job-name=myjob path/to/job.sh` : submit a batch job with a custom name
- `sbatch --time=00:30:00 path/to/job.sh` : submit a batch job with a time limit of 30 minutes
- `sbatch --nodes=3 path/to/job.sh` : submit a job and request multiple nodes

## Categories
processes, files

## Links
- More information: https://manned.org/sbatch
- tldr page: https://tldr.inbrowser.app/pages/linux/sbatch
