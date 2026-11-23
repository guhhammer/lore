# dolt clone

Clone a repository into a new directory.

## Usage
- `dolt clone repository_url path/to/directory` : clone an existing repository into a specific directory (defaults to the repository name)
- `dolt clone --remote remote_name repository_url` : clone an existing repository and add a specific remote (defaults to origin)
- `dolt clone -b branch_name repository_url` : clone an existing repository only fetching a specific branch (defaults to all branches)
- `dolt clone --aws-region region_name repository_url` : clone a repository, using an AWS region (uses the profile's default region if none is provided)
- `dolt clone --aws-creds-file credentials_file repository_url` : clone a repository, using an AWS credentials file
- `dolt clone --aws-creds-profile profile_name repository_url` : clone a repository, using an AWS credentials profile (uses the default profile if none is provided)
- `dolt clone --aws-creds-type credentials_type repository_url` : clone a repository, using an AWS credentials type

## Categories
packages, cloud, files

## Links
- More information: https://docs.dolthub.com/cli-reference/cli#dolt-clone
- tldr page: https://tldr.inbrowser.app/pages/common/dolt-clone
