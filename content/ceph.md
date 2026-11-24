# ceph

A unified storage system.

## Usage
- `ceph status` : check cluster health status
- `ceph df` : check cluster usage stats
- `ceph pg dump --format plain` : get the statistics for the placement groups in a cluster
- `ceph osd pool create pool_name page_number` : create a storage pool
- `ceph osd pool delete pool_name` : delete a storage pool
- `ceph osd pool rename current_name new_name` : rename a storage pool
- `ceph pg repair pool_name` : self-repair pool storage

## Categories
containers

## Links
- More information: https://docs.ceph.com/en/latest/man/8/ceph/
- tldr page: https://tldr.inbrowser.app/pages/linux/ceph
- Wikipedia: https://en.wikipedia.org/wiki/Ceph_(software)
