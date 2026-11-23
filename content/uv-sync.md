# uv sync

Update the project's environment to match the lockfile.

## Usage
- `uv sync` : sync the project environment with the lockfile
- `uv sync --all-extras` : sync and include all optional dependencies
- `uv sync --extra extra_name` : sync with specific optional dependencies
- `uv sync --only-dev` : sync only development dependencies
- `uv sync --no-dev` : sync excluding development dependencies
- `uv sync --group group_name` : sync specific dependency groups
- `uv sync --check` : check if environment is already synchronized (no changes)
- `uv sync --dry-run` : simulate syncing, showing what would be synced without making changes

## Categories
development, packages

## Links
- More information: https://docs.astral.sh/uv/reference/cli/#uv-sync
- tldr page: https://tldr.inbrowser.app/pages/common/uv-sync
