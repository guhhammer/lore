# minio server

MinIO server command for starting the MinIO S3 compatible storage engine.

## Usage
- `minio server path/to/data_directory` : start a server using the default configuration
- `minio server --address ":port" --console-address ":port" path/to/data_directory` : start a server binding to a different port for the API and web console
- `minio server node1_hostname path/to/data_directory node2_hostname path/to/data_directory` : start a server as part of a cluster of 2 nodes

## Categories
network, files, cloud

## Links
- More information: https://docs.min.io/enterprise/aistor-object-store/reference/aistor-server/
- tldr page: https://tldr.inbrowser.app/pages/common/minio-server
