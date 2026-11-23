# id

Display current user and group identity.

## Usage
- `id` : display current user's ID (UID), group ID (GID), and groups to which they belong
- `id -un` : display the current user identity
- `id -u` : display the current user identity as a number
- `id -gn` : display the current primary group identity
- `id -g` : display the current primary group identity as a number
- `id -Gn` : display all groups the current user belongs to
- `id username` : display an arbitrary user's ID (UID), group ID (GID), and groups to which they belong
- `id +uid_number` : skip name lookup and specify the UID number explicitly

## See also
- logname

## Categories
users

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/id-invocation.html
- tldr page: https://tldr.inbrowser.app/pages/common/id
