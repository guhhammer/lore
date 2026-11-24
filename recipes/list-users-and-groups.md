# list users and groups

See the accounts on this machine, the groups, and who is in each.

## Steps
- `getent passwd | cut -d: -f1` : every account's name
- `getent passwd | awk -F: '$3 >= 1000 {print $1}'` : the people, not the system accounts (Linux)
- `getent group` : every group and its members
- `groups NAME` : the groups of one user
- `id` : your own user, groups and numbers
- `dscl . list /Users` : macOS

## See also
- getent
- groups
- id

## Categories
users system
