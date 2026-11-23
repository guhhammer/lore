# az tag

Manage tags on a resource. Part of `azure-cli` (also known as `az`).

## Usage
- `az tag add-value -n tag_name --value tag_value` : create a tag value
- `az tag create -n tag_name` : create a tag in the subscription
- `az tag delete -n tag_name` : delete a tag from the subscription
- `az tag list --resource-id /subscriptions/subscription_id` : list all tags on a subscription
- `az tag remove-value -n tag_name --value tag_value` : delete a tag value for a specific tag name

## Categories
cloud

## Links
- More information: https://learn.microsoft.com/cli/azure/tag
- tldr page: https://tldr.inbrowser.app/pages/common/az-tag
