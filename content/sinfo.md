# sinfo

View information about Slurm nodes and partitions.

## Usage
- `sinfo -s` : show a quick summary overview of the cluster
- `sinfo` : view the detailed status of all partitions across the entire cluster
- `sinfo -p partition_name` : view the detailed status of a specific partition
- `sinfo -t idle` : view information about idle nodes
- `sinfo -d` : summarize dead nodes
- `sinfo -R` : list dead nodes and the reasons why

## See also
- squeue
- sbatch

## Links
- More information: https://slurm.schedmd.com/sinfo.html
- tldr page: https://tldr.inbrowser.app/pages/linux/sinfo
