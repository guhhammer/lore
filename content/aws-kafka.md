# aws kafka

Manage an Amazon MSK (Managed Streaming for Apache Kafka) cluster.

## Usage
- `aws kafka create-cluster --cluster-name cluster_name --broker-node-group-info instanceType=instance_type,clientSubnets=subnet_id1 subnet_id2 ... --kafka-version version --number-of-broker-nodes number` : create a new MSK cluster
- `aws kafka describe-cluster --cluster-arn cluster_arn` : describe an MSK cluster
- `aws kafka list-clusters` : list all MSK clusters in the current region
- `aws kafka create-configuration --name configuration_name --server-properties file://path/to/configuration_file.txt` : create a new MSK configuration
- `aws kafka describe-configuration --arn configuration_arn` : describe an MSK configuration
- `aws kafka list-configurations` : list all MSK configurations in the current region
- `aws kafka update-cluster-configuration --cluster-arn cluster_arn --configuration-info arn=configuration_arn,revision=configuration_revision` : update the MSK cluster configuration
- `aws kafka delete-cluster --cluster-arn cluster_arn` : delete the MSK cluster

## Categories
containers, cloud

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/kafka/
- tldr page: https://tldr.inbrowser.app/pages/common/aws-kafka
