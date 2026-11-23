# sattach

Attach to a Slurm job step.

## Usage
- `sattach jobid.stepid` : redirect the IO streams (`stdout`, `stderr`, and `stdin`) of a Slurm job step to the current terminal
- `sattach --input-filter task_number` : use the current console's input as `stdin` to the specified task
- `sattach --output|error-filter task_number` : only redirect `stdin`/`stderr` of the specified task

## Categories
processes

## Links
- More information: https://slurm.schedmd.com/sattach.html
- tldr page: https://tldr.inbrowser.app/pages/linux/sattach
