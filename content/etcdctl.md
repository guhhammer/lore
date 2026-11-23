# etcdctl

Interact with `etcd`, a highly-available key-value pair store.

## Usage
- `etcdctl get my/key` : display the value associated with a specified key
- `etcdctl put my/key my_value` : store a key-value pair
- `etcdctl del my/key` : delete a key-value pair
- `etcdctl <path/to/file.txt put my/file` : store a key-value pair, reading the value from a file
- `etcdctl snapshot save path/to/snapshot.db` : save a snapshot of the etcd keystore
- `etcdctl snapshot restore path/to/snapshot.db` : restore a snapshot of an etcd keystore (restart the etcd server afterwards)
- `etcdctl user add my_user` : add a user
- `etcdctl watch my/key` : watch a key for changes

## Categories
files

## Links
- More information: https://etcd.io/docs/latest/dev-guide/interacting_v3/
- tldr page: https://tldr.inbrowser.app/pages/common/etcdctl
