# gh secret set

Create or update GitHub secrets.

## Usage
- `gh secret set name` : set a secret for the current repository (user will be prompted for the value)
- `gh <path/to/file secret set name` : set a secret from a file for the current repository
- `gh secret set name -b value -R owner/repository` : set a secret for a specific repository
- `gh secret set name -o organization -r "repository1,repository2,..."` : set an organization secret for specific repositories
- `gh secret set name -o organization -v all|private|selected` : set an organization secret with a specific visibility

## Categories
packages, files

## Links
- More information: https://cli.github.com/manual/gh_secret_set
- tldr page: https://tldr.inbrowser.app/pages/common/gh-secret-set
