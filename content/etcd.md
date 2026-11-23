# etcd

A distributed, reliable key-value store for the most critical data of a distributed system.

## Usage
- `etcd` : start a single-node etcd cluster
- `etcd --advertise-client-urls http://127.0.0.1:1234 --listen-client-urls http://127.0.0.1:1234` : start a single-node etcd cluster, listening for client requests on a custom URL
- `etcd --name my_etcd_cluster` : start a single-node etcd cluster with a custom name
- `etcd --enable-pprof --metrics extensive` : start a single-node etcd cluster with extensive metrics available at <http://localhost:2379/debug/pprof/>

## Categories
network, containers

## Links
- More information: https://etcd.io/docs/latest/op-guide/configuration/#command-line-flags
- tldr page: https://tldr.inbrowser.app/pages/common/etcd
