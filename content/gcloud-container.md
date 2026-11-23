# gcloud container

Manage containerized applications on Kubernetes and clusters.

## Usage
- `gcloud auth configure-docker` : register `gcloud` as a Docker credential helper
- `gcloud container clusters create cluster_name` : create a cluster to run GKE containers
- `gcloud container clusters list` : list clusters for running GKE containers
- `gcloud container clusters get-credentials cluster_name` : update kubeconfig to get `kubectl` to use a GKE cluster
- `gcloud container images list-tags image` : list tag and digest metadata for a container image
- `gcloud container clusters describe cluster_name` : describe an existing cluster for running containers

## Categories
containers, cloud, media

## Links
- More information: https://docs.cloud.google.com/sdk/gcloud/reference/container
- tldr page: https://tldr.inbrowser.app/pages/common/gcloud-container
