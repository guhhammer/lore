# npm unstar

Remove the favorite/star mark from a package.

## Usage
- `npm unstar package_name` : unstar a public package from the default registry
- `npm unstar @scope/package_name` : unstar a package within a specific scope
- `npm unstar package_name --registry registry_url` : unstar a package from a specific registry
- `npm unstar package_name --auth-type legacy|oauth|web|saml` : unstar a private package that requires authentication
- `npm unstar package_name --otp otp` : unstar a package by providing an OTP for two-factor authentication
- `npm unstar package_name --loglevel silent|error|warn|notice|http|timing|info|verbose|silly` : unstar a package with a specific logging level

## Categories
packages, network

## Links
- More information: https://docs.npmjs.com/cli/v11/npm-unstar
- tldr page: https://tldr.inbrowser.app/pages/common/npm-unstar
- Wikipedia: https://en.wikipedia.org/wiki/Npm
