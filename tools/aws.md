# aws

aws is the command line of Amazon Web Services. This page covers signing in, S3 files, servers (EC2), identity and a few everyday lookups.

## Tasks
### Set up
- `aws configure` : save an access key, secret and default region
- `aws configure sso` : set up a profile that signs in through IAM Identity Center
- `aws sso login --profile NAME` : sign in with an SSO profile
- `aws sts get-caller-identity` : who you are signed in as
- `aws configure list-profiles` : profiles on this machine
- `export AWS_PROFILE=NAME` : use a profile for this shell
### S3 files
- `aws s3 ls` : list buckets
- `aws s3 ls s3://BUCKET/PATH/` : list a bucket's files
- `aws s3 cp FILE s3://BUCKET/PATH/` : upload a file
- `aws s3 cp s3://BUCKET/PATH/FILE .` : download a file
- `aws s3 sync DIR s3://BUCKET/PATH` : upload what changed in a folder
- `aws s3 sync s3://BUCKET/PATH DIR` : download what changed
- `aws s3 rm s3://BUCKET/PATH --recursive` : delete everything under a path (deletes it)
- `aws s3 presign s3://BUCKET/PATH/FILE --expires-in 3600` : a temporary download link
### EC2 servers
- `aws ec2 describe-instances --query 'Reservations[].Instances[].[InstanceId,State.Name,PublicIpAddress]' --output table` : instances, state and IP
- `aws ec2 start-instances --instance-ids ID` : start an instance
- `aws ec2 stop-instances --instance-ids ID` : stop it
- `aws ec2 describe-security-groups` : firewall groups
### Other everyday services
- `aws logs tail /aws/lambda/NAME --follow` : follow a log group
- `aws lambda invoke --function-name NAME out.json` : call a Lambda function
- `aws ecr get-login-password | docker login --username AWS --password-stdin ACCOUNT.dkr.ecr.REGION.amazonaws.com` : sign Docker in to ECR
- `aws secretsmanager get-secret-value --secret-id NAME` : read a secret
- `aws ssm start-session --target ID` : open a shell on an instance (needs the session plugin)
### Output
- `aws ... --output json` : JSON, the default (also table, text, yaml)
- `aws ... --query 'Path[].Field'` : pick fields with a JMESPath expression
- `aws ... --region REGION` : run in another region
- `aws help` : list services; add help after any command for its manual

## Notes
- Keys live in `~/.aws/credentials`: never commit them. Prefer SSO or roles over long-lived keys.
- Most resources are per region: check `aws configure get region` when something seems missing.
- `s3 rm --recursive` and `terminate-instances` cannot be undone: add `--dryrun` to s3 commands first.
- Install version 2 from the official page in Links.

## See also
- terraform
- gcloud
- az
- docker

## Links
- Reference: https://docs.aws.amazon.com/cli/latest/reference/
- Install: https://aws.amazon.com/cli/

## Categories
cloud
