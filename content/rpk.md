# rpk

Manage Redpanda topics, clusters, groups, security, and more via a single binary.

## Usage
- `rpk topic create topic_name` : create a new topic
- `rpk topic produce topic_name` : produce a message to a topic
- `rpk topic consume topic_name1 topic_name2 ...` : consume messages from multiple topics
- `rpk topic list` : list all topics
- `rpk cluster info` : display cluster information
- `rpk group list` : list all consumer groups
- `rpk group describe group_name` : describe a consumer group with lag information
- `rpk version` : display version

## Categories
users

## Links
- More information: https://docs.redpanda.com/current/reference/rpk/
- tldr page: https://tldr.inbrowser.app/pages/common/rpk
