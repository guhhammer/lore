# sdiag

Show information about the execution of `slurmctld`.

## Usage
- `sdiag -a` : show all performance counters related to the execution of `slurmctld`
- `sdiag -r` : reset performance counters related to the execution of `slurmctld`
- `sdiag -a --json|yaml` : specify the output format
- `sdiag -a -M cluster_name` : specify the cluster to send commands to

## Links
- More information: https://slurm.schedmd.com/sdiag.html
- tldr page: https://tldr.inbrowser.app/pages/linux/sdiag
