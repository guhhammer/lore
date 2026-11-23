# odpscmd auth

User authorities in ODPS (Open Data Processing Service).

## Usage
- `add user username;` : [Interactive] Add a user to the current project
- `grant action_list on object_type object_name to user username;` : [Interactive] Grant a set of authorities to a user
- `show grants for username;` : [Interactive] Show authorities of a user
- `create role role_name;` : [Interactive] Create a user role
- `grant action_list on object_type object_name to role role_name;` : [Interactive] Grant a set of authorities to a role
- `desc role role_name;` : [Interactive] Describe authorities of a role
- `grant role_name to username;` : [Interactive] Grant a role to a user

## Categories
users, system

## Links
- More information: https://www.alibabacloud.com/help/en/maxcompute/user-guide/maxcompute-client
- tldr page: https://tldr.inbrowser.app/pages/common/odpscmd-auth
