# aws lightsail

Manage Amazon Lightsail resources.

## Usage
- `aws lightsail get-instances` : list all virtual private servers, or instances
- `aws lightsail list-bundles` : list all bundles (instance plans)
- `aws lightsail list-blueprints` : list all available instance images, or blueprints
- `aws lightsail create-instances --instance-names name --availability-zone region --bundle-id nano_2_0 --blueprint-id blueprint_id` : create an instance
- `aws lightsail get-instance-state --instance-name name` : print the state of a specific instance
- `aws lightsail stop-instance --instance-name name` : stop a specific instance
- `aws lightsail delete-instance --instance-name name` : delete a specific instance

## Categories
cloud

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/lightsail/
- tldr page: https://tldr.inbrowser.app/pages/common/aws-lightsail
