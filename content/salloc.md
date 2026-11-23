# salloc

Start an interactive shell session or execute a command by allocating one or more nodes in a cluster.

## Usage
- `salloc` : start an interactive shell session on a node in the cluster
- `salloc ls --all` : execute the specified command synchronously on a node in the cluster
- `salloc -C (amd|intel)&gpu` : only allocate nodes fulfilling the specified constraints

## Categories
containers, shell

## Links
- More information: https://slurm.schedmd.com/salloc.html
- tldr page: https://tldr.inbrowser.app/pages/linux/salloc
