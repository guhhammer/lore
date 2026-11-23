# gcloud compute

Create, run, and manage VMs on Google Cloud infrastructure.

## Usage
- `gcloud compute zones list` : list Compute Engine zones
- `gcloud compute instances create instance_name` : create a VM instance
- `gcloud compute instances describe instance_name` : display a VM instance's details
- `gcloud compute instances list` : list all VM instances in a project
- `gcloud compute disks snapshot disk_name --snapshot-names snapshot_name` : create a snapshot of a persistent disk
- `gcloud compute snapshots describe snapshot_name` : display a snapshot's details
- `gcloud compute snapshots delete snapshot_name` : delete a snapshot
- `gcloud compute ssh instance_name` : connect to a VM instance using SSH

## Categories
cloud, network

## Links
- More information: https://docs.cloud.google.com/sdk/gcloud/reference/compute
- tldr page: https://tldr.inbrowser.app/pages/common/gcloud-compute
