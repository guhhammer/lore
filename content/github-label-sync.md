# github-label-sync

Synchronize GitHub labels.

## Usage
- `github-label-sync --access-token token repository_name` : synchronize labels using a local `labels.json` file
- `github-label-sync --access-token token --labels url|path/to/json_file repository_name` : synchronize labels using a specific labels JSON file
- `github-label-sync --access-token token --dry-run repository_name` : simulate synchronizing labels without actually doing it
- `github-label-sync --access-token token --allow-added-labels repository_name` : keep labels that aren't in `labels.json`
- `github-label-sync repository_name` : synchronize using the `$GITHUB_ACCESS_TOKEN` environment variable

## Categories
files, network, text

## Links
- More information: https://github.com/Financial-Times/github-label-sync
- tldr page: https://tldr.inbrowser.app/pages/common/github-label-sync
