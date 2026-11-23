# uv cache

Manage `uv`'s global cache directory.

## Usage
- `uv cache dir` : show the cache directory path
- `uv cache clean` : clean the entire cache (removes all cached packages and environments)
- `uv cache clean package1 package2 ...` : clean the cache for specific packages
- `uv cache prune` : prune all unreachable objects from the cache
- `uv cache prune --ci` : prune cache optimized for CI environments like GitHub Actions
- `uv cache clean --cache-dir path/to/cache` : use a specific cache directory
- `uv cache clean -v` : clean cache with verbose output

## Categories
files

## Links
- More information: https://docs.astral.sh/uv/reference/cli/#uv-cache
- tldr page: https://tldr.inbrowser.app/pages/common/uv-cache
