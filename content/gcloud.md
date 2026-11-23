# gcloud

The official CLI tool for Google Cloud Platform. Some subcommands such as `app`, `init`, etc. have their own usage documentation.

## Usage
- `gcloud config list` : list all properties in one's active configuration
- `gcloud auth login` : log in to a Google account
- `gcloud config set project project_name` : set the active project
- `gcloud compute ssh user@instance` : SSH into a virtual machine instance
- `gcloud compute instances list` : display all Google Compute Engine instances in a project (by default instances from all zones are listed)
- `gcloud container clusters get-credentials cluster_name` : update a kubeconfig file with the appropriate credentials to point `kubectl` to a specific cluster in Google Kubernetes Engine (GKE)
- `gcloud components update` : update all `gcloud` components
- `gcloud help command` : display help for a given command

## Categories
cloud

## Links
- More information: https://docs.cloud.google.com/sdk/gcloud
- tldr page: https://tldr.inbrowser.app/pages/common/gcloud
