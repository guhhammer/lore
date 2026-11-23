# cephadm

Deploy and manage a Ceph cluster using containers. Part of the Ceph orchestrator framework.

## Usage
- `sudo cephadm bootstrap --mon-ip monitor_ip` : bootstrap a new Ceph cluster on the current host
- `sudo cephadm add-host hostname ip_address` : add a new host to the cluster
- `sudo cephadm deploy service_type --name service_name` : deploy a specific service (e.g., mgr, mon, osd)
- `sudo cephadm shell -- ceph -s` : check the status of cluster services
- `sudo cephadm shell` : enter a shell environment inside the Ceph container
- `sudo cephadm rm-service service_type --name service_name` : remove a service from the cluster

## Categories
containers, users, network

## Links
- More information: https://docs.ceph.com/en/latest/man/8/cephadm/
- tldr page: https://tldr.inbrowser.app/pages/linux/cephadm
