# pio org

Manage PlatformIO organizations and their owners.

## Usage
- `pio org create organization_name` : create a new organization
- `pio org destroy organization_name` : delete an organization
- `pio org add organization_name username` : add a user to an organization
- `pio org remove organization_name username` : remove a user from an organization
- `pio org list` : list all organizations the current user is a member of and their owners
- `pio org update --orgname new_organization_name --email new_email --displayname new_display_name organization_name` : update the name, email, or display name of an organization

## Categories
users

## Links
- More information: https://docs.platformio.org/en/latest/core/userguide/org/
- tldr page: https://tldr.inbrowser.app/pages/common/pio-org
