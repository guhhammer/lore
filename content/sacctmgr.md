# sacctmgr

View, set up, and manage Slurm accounts.

## Usage
- `sacctmgr show configuration` : show current configuration
- `sacctmgr add cluster cluster_name` : add a cluster to the slurm database
- `sacctmgr add account account_name cluster=cluster_of_account` : add an account to the slurm database
- `sacctmgr show user|association|cluster|account format="Account%10" format="GrpTRES%30"` : show details of user/association/cluster/account using a specific format

## Categories
users, containers

## Links
- More information: https://slurm.schedmd.com/sacctmgr.html
- tldr page: https://tldr.inbrowser.app/pages/linux/sacctmgr
