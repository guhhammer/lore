# stolonctl

CLI for Stolon, a cloud native PostgreSQL manager for PostgreSQL high availability.

## Usage
- `stolonctl --cluster-name cluster_name --store-backend store_backend --store-endpoints store_endpoints status` : get cluster status
- `stolonctl --cluster-name cluster_name --store-backend store_backend --store-endpoints store_endpoints clusterdata` : get cluster data
- `stolonctl --cluster-name cluster_name --store-backend store_backend --store-endpoints store_endpoints spec` : get cluster specification
- `stolonctl --cluster-name cluster_name --store-backend store_backend --store-endpoints store_endpoints update --patch 'cluster_spec'` : update cluster specification with a patch in JSON format

## Categories
containers, cloud

## Links
- More information: https://github.com/sorintlab/stolon/blob/master/doc/commands/stolonctl.md
- tldr page: https://tldr.inbrowser.app/pages/common/stolonctl
