# noseyparker

Scan text and Git history for secrets and sensitive information. Note: Use a separate datastore directory (`--datastore`) for each scan.

## Usage
- `noseyparker scan path/to/file_or_directory -d path/to/datastore.np` : scan a local file or directory for secrets
- `noseyparker report -d path/to/datastore.np` : show a report from a previous scan
- `noseyparker report -d path/to/datastore.np -f human|json|jsonl|sarif` : show a report in different format (default is `human`)
- `noseyparker scan --git-url url -d path/to/datastore.np` : scan a remote Git repo (and Git history) for secrets
- `noseyparker scan --github-user|--github-organization username_or_org_name -d path/to/datastore.np` : scan all GitHub repositories of a user or organization for secrets
- `noseyparker rules list` : list all scan rules
- `noseyparker github repos list --user|--organization username_or_org_name` : list all GitHub repositories of a user or organization

## See also
- trufflehog

## Categories
files, version-control, network

## Links
- More information: https://github.com/praetorian-inc/noseyparker#usage-examples
- tldr page: https://tldr.inbrowser.app/pages/common/noseyparker
