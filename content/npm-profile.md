# npm profile

Manage the npm profile and related settings. Note: This command is unaware of workspaces.

## Usage
- `npm profile get` : view the npm profile details
- `npm profile get property` : view a specific property of the profile
- `npm profile set property value` : set or update a profile property
- `npm profile set email email` : set the public email address
- `npm profile set fullname name` : set the public name
- `npm profile set password` : set a new password interactively
- `npm profile enable-2fa auth-only|auth-and-writes` : enable two-factor authentication (2FA) (defaults to `auth-and-writes`)
- `npm profile disable-2fa` : disable two-factor authentication (2FA)

## Links
- More information: https://docs.npmjs.com/cli/npm-profile/
- tldr page: https://tldr.inbrowser.app/pages/common/npm-profile
