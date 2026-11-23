# aws ssm

Securely interact with and manage AWS resources. Note: Interactive sessions require the Session Manager plugin to be installed.

## Usage
- `aws ssm send-command --instance-ids instance_id1 instance_id2 ... --document-name "AWS-RunShellScript" --parameters 'commands=["command"]'` : run commands on instances
- `aws ssm list-command-invocations --instance-id "instance_id"` : check command invocations on an instance
- `aws ssm list-command-invocations --command-id "command_id" --details` : check command output for a specific command invocation
- `aws ssm start-session --target "instance_id"` : start an interactive session with an instance
- `aws ssm start-session --target "instance_id" --document-name "AWS-StartPortForwardingSessionToRemoteHost" --parameters '{"portNumber":["remote_port"],"localPortNumber":["local_port"]}'` : start a port forwarding session to a remote host

## Categories
cloud, network

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/ssm/
- tldr page: https://tldr.inbrowser.app/pages/common/aws-ssm
