# apptainer cache

Manage the local Apptainer cache.

## Usage
- `apptainer cache list` : list all cached container images
- `apptainer cache list -v` : list cached container images with detailed information
- `apptainer cache list -T library|oci|shub|blob|...` : list only a specific cache type
- `apptainer cache clean` : clean the entire cache
- `apptainer cache clean -T library|oci|shub|blob|...` : clean only a specific cache type
- `apptainer cache clean -D days` : clean cache entries older than a specific number of days
- `apptainer cache clean -n` : simulate cleaning the cache, showing what would be removed
- `apptainer cache clean -f` : force clean without confirmation

## Categories
containers

## Links
- More information: https://apptainer.org/docs/user/main/cli/apptainer_cache.html
- tldr page: https://tldr.inbrowser.app/pages/linux/apptainer-cache
