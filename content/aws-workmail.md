# aws workmail

Manage Amazon WorkMail.

## Usage
- `aws workmail list-organizations` : list all WorkMail organizations
- `aws workmail list-users --organization-id organization_id` : list all users of a specific organization
- `aws workmail create-user --name username --display-name name --password password --organization-id organization_id` : create a WorkMail user in a specific organization
- `aws workmail register-to-work-mail --entity-id entity_id --email email --organization-id organization_id` : register and enable a group/user to WorkMail
- `aws workmail create-group --name group_name --organization-id organization_id` : create a WorkMail group in a specific organization
- `aws workmail associate-member-to-group --group-id group_id --member-id member_id --organization-id organization_id` : associate a member to a specific group
- `aws workmail deregister-from-work-mail --entity-id entity_id --organization-id organization_id` : deregister and disable a user/group from WorkMail
- `aws workmail delete-user --user-id user_id --organization-id organization_id` : delete a user from an organization

## Categories
users, cloud

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/workmail/
- tldr page: https://tldr.inbrowser.app/pages/common/aws-workmail
