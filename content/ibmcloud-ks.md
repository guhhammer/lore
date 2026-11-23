# ibmcloud ks

Manage Kubernetes and OpenShift clusters in IBM Cloud.

## Usage
- `ibmcloud ks cluster get -c cluster_id` : view the details of a cluster
- `ibmcloud ks cluster ca status -c cluster_id` : view the rotation status of Cluster Authority certificates for a cluster
- `ibmcloud ks worker-pool ls -c cluster_id` : view worker pools of a cluster
- `ibmcloud ks worker replace -c cluster_id -w worker_id` : delete a worker node and replace it with a new one in the same worker pool
- `ibmcloud ks help` : list all actions available under this command

## Categories
containers, cloud

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/ibmcloud-ks
