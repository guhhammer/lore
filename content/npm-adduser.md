# npm adduser

Add a registry user account.

## Usage
- `npm adduser --registry registry_url` : create a new user in the specified registry and save credentials to `.npmrc`
- `npm login --scope @organization --registry https://registry.example.com` : log in to a private registry with a specific scope
- `npm logout --scope @organization` : log out from a specific scope and remove the auth token
- `npm init --scope @organization -y` : create a scoped package during initialization

## Categories
users, network

## Links
- More information: https://docs.npmjs.com/cli/v11/npm-adduser
- tldr page: https://tldr.inbrowser.app/pages/common/npm-adduser
- Wikipedia: https://en.wikipedia.org/wiki/Npm
