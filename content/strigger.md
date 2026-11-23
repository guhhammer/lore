# strigger

View or modify Slurm trigger information. Triggers are actions that are automatically run when an event occurs on a Slurm cluster.

## Usage
- `strigger --set --primary_database_failure|primary_slurmdbd_failure|primary_slurmctld_acct_buffer_full|primary_slurmctld_failure|... -p path/to/executable` : register a new trigger. Execute the specified program when the specified event occurs
- `strigger --set -j job_id -f -p "path/to/executable argument1 argument2 ..."` : execute the specified program when the specified job terminated
- `strigger --get` : view active triggers
- `strigger --get -j job_id` : view active triggers regarding the specified job
- `strigger --clear trigger_id` : clear the specified trigger

## Categories
containers

## Links
- More information: https://slurm.schedmd.com/strigger.html
- tldr page: https://tldr.inbrowser.app/pages/linux/strigger
