# lxc profile

Manage profiles for LXD containers.

## Usage
- `lxc profile list` : list all available profiles
- `lxc profile show profile_name` : show the configuration of a specific profile
- `lxc profile edit profile_name` : edit a specific profile in the default editor
- `lxc <config.yaml profile edit profile_name` : edit a specific profile importing the configuration values from a file
- `lxc launch container_image container_name -p profile1 -p profile2` : launch a new container with specific profiles
- `lxc profile assign container_name profile1,profile2` : change the profiles of a running container

## Categories
containers

## Links
- More information: https://canonical.com/lxd/docs/latest/reference/manpages/lxc/profile/
- tldr page: https://tldr.inbrowser.app/pages/linux/lxc-profile
