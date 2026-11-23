# srun

Run a command under the Slurm workload manager.

## Usage
- `srun hostname` : run a simple command interactively
- `srun -n 4 path/to/program` : run a job with 4 tasks (CPUs)
- `srun --mem 8G path/to/program` : allocate 8 GB of memory
- `srun -p gpu path/to/program` : run a job on a specific partition
- `srun path/to/program >path/to/output` : run a job and save the output to a file

## Categories
files, processes

## Links
- More information: https://slurm.schedmd.com/srun.html
- tldr page: https://tldr.inbrowser.app/pages/common/srun
