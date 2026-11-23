# awslogs

Query groups, streams, and events from Amazon CloudWatch logs.

## Usage
- `awslogs groups` : list log groups
- `awslogs streams /var/log/syslog` : list existing streams for the specified group
- `awslogs get /var/log/syslog -s '2h ago' -e '1h ago'` : get logs for any streams in the specified group between 1 and 2 hours ago
- `awslogs get /aws/lambda/my_lambda_group --filter-pattern 'ERROR'` : get logs that match a specific CloudWatch Logs Filter pattern
- `awslogs get /var/log/syslog ALL --watch` : watch logs for any streams in the specified group

## Categories
users, monitoring, databases

## Links
- More information: https://github.com/jorgebastida/awslogs#options
- tldr page: https://tldr.inbrowser.app/pages/common/awslogs
