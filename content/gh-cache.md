# gh cache

Manage GitHub Actions caches.

## Usage
- `gh cache ls` : list caches for the current repository
- `gh cache ls -R owner/repository` : list caches for a specific repository
- `gh cache ls -k key_prefix` : list caches with a specific cache key prefix
- `gh cache ls -r refs/heads/branch_name` : list caches for a specific branch
- `gh cache ls -S last_accessed_at -O asc` : list caches sorted by least recently accessed
- `gh cache delete cache_id` : delete a cache by id
- `gh cache delete cache_key` : delete a cache by key
- `gh cache delete -a` : delete all caches

## Categories
packages

## Links
- More information: https://cli.github.com/manual/gh_cache
- tldr page: https://tldr.inbrowser.app/pages/common/gh-cache
