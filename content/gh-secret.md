# gh secret

Manage GitHub secrets.

## Usage
- `gh secret ls` : list secret keys for the current repository
- `gh secret ls -o organization` : list secret keys for a specific organization
- `gh secret ls -R owner/repository` : list secret keys for a specific repository
- `gh secret set name` : set a secret for the current repository (user will be prompted for the value)
- `gh <path/to/file secret set name` : set a secret from a file for the current repository
- `gh secret set name -o organization -r repository1,repository2` : set an organization secret for specific repositories
- `gh secret remove name` : remove a secret for the current repository
- `gh secret remove name -o organization` : remove a secret for a specific organization

## Categories
packages, files

## Links
- More information: https://cli.github.com/manual/gh_secret
- tldr page: https://tldr.inbrowser.app/pages/common/gh-secret
