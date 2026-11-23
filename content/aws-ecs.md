# aws ecs

Manage Elastic Container Service (ECS) clusters.

## Usage
- `aws ecs list-task-definitions` : list task definitions
- `aws ecs describe-task-definition --task-definition task_definition` : describe a task definition
- `aws ecs register-task-definition --cli-input-json file://path/to/file.json` : create a task definition or make a revision
- `aws ecs deregister-task-definition --task-definition task_definition:revision_number` : deregister a task definition marking them INACTIVE
- `aws ecs list-services --cluster cluster_name` : list services in a specific cluster
- `aws ecs describe-services --services service_name service_name --cluster cluster_name` : describe one or more services in a cluster
- `aws ecs update-service --cluster cluster_name --service service_name --task-definition task_definition_arn --force-new-deployment` : update a service and force new deployment
- `aws ecs wait services-stable --cluster cluster_name --services service_name service_name` : wait until one or more services to succeed

## Categories
system, cloud, containers

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/ecs/
- tldr page: https://tldr.inbrowser.app/pages/common/aws-ecs
