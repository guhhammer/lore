# pio access

Set the access level on published resources (packages) in the registry.

## Usage
- `pio access grant guest|maintainer|admin username resource_urn` : grant a user access to a resource
- `pio access revoke username resource_urn` : remove a user's access to a resource
- `pio access list username` : show all resources that a user or team has access to and the access level
- `pio access private resource_urn` : restrict access to a resource to specific users or team members
- `pio access public resource_urn` : allow all users access to a resource

## Categories
users, packages

## Links
- More information: https://docs.platformio.org/en/latest/core/userguide/access/
- tldr page: https://tldr.inbrowser.app/pages/common/pio-access
