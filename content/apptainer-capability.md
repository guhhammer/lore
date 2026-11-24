# apptainer capability

Manage Linux capabilities for users and groups.

## Usage
- `apptainer capability avail` : show all available Linux capabilities
- `apptainer capability avail cap_chown,cap_net_raw,...` : show descriptions for specific capabilities
- `apptainer capability list` : list capabilities for all users and groups
- `apptainer capability list username|groupname` : list capabilities for a specific user or group
- `sudo apptainer capability add -u username cap_net_raw,cap_chown,...` : add capabilities to a user
- `sudo apptainer capability add -g groupname cap_net_raw,cap_chown,...` : add capabilities to a group
- `sudo apptainer capability drop -u username cap_net_raw,cap_chown,...` : remove capabilities from a user
- `sudo apptainer capability drop -u username all` : remove all capabilities from a user

## Categories
users

## Links
- More information: https://apptainer.org/docs/user/main/cli/apptainer_capability.html
- tldr page: https://tldr.inbrowser.app/pages/linux/apptainer-capability
- Wikipedia: https://en.wikipedia.org/wiki/Apptainer
