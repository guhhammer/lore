# aws cloudformation

Model, provision, and manage AWS and third-party resources by treating infrastructure as code.

## Usage
- `aws cloudformation create-stack --stack-name stack_name --region region --template-body file://path/to/file.yml --profile profile` : create a stack from a template file
- `aws cloudformation delete-stack --stack-name stack_name --profile profile` : delete a stack
- `aws cloudformation list-stacks --profile profile` : list all stacks
- `aws cloudformation list-stacks --stack-status-filter CREATE_COMPLETE --profile profile` : list all running stacks
- `aws cloudformation describe-stacks --stack-name stack_id --profile profile` : check the status of a stack
- `aws cloudformation detect-stack-drift --stack-name stack_id --profile profile` : initiate drift detection for a stack
- `aws cloudformation describe-stack-resource-drifts --stack-name stack_drift_detection_id --profile profile` : check the drift status output of a stack using `StackDriftDetectionId` from the previous command output

## Categories
cloud, files

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/cloudformation/
- tldr page: https://tldr.inbrowser.app/pages/common/aws-cloudformation
