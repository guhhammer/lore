# aws ec2

Manage AWS EC2 instances and volumes. Provides secure and resizable computing capacity in the AWS cloud for faster development and deployment of applications.

## Usage
- `aws ec2 describe-instances --instance-ids instance_id` : display information about a specific instance
- `aws ec2 describe-instances` : display information about all instances
- `aws ec2 describe-volumes` : display information about all EC2 volumes
- `aws ec2 delete-volume --volume-id volume_id` : delete an EC2 volume
- `aws ec2 create-snapshot --volume-id volume_id` : create a snapshot from an EC2 volume
- `aws ec2 describe-images` : list available AMIs (Amazon Machine Images)
- `aws ec2 help` : show list of all available EC2 commands
- `aws ec2 subcommand help` : display help for specific EC2 subcommand

## Categories
cloud, disks, desktop

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/ec2/
- tldr page: https://tldr.inbrowser.app/pages/common/aws-ec2
