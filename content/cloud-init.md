# cloud-init

Manage cloud instance initialization.

## Usage
- `cloud-init status` : display the status of the most recent cloud-init run
- `cloud-init status --wait` : wait for cloud-init to finish running and then report status
- `cloud-init query --list-keys` : list available top-level metadata keys to query
- `cloud-init query dot_delimited_variable_path` : query cached instance metadata for data
- `cloud-init clean` : clean logs and artifacts to allow cloud-init to rerun

## Categories
cloud, databases

## Links
- More information: https://cloudinit.readthedocs.io/en/latest/reference/cli.html
- tldr page: https://tldr.inbrowser.app/pages/linux/cloud-init
