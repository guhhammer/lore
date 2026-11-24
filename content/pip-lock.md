# pip lock

Lock Python packages and their dependencies into a reproducible file. Experimental feature of `pip`.

## Usage
- `pip lock -e .` : generate a `pylock.toml` for the current project
- `pip lock -r path/to/requirements.txt` : lock dependencies from a requirements file
- `pip lock -o path/to/lockfile.toml` : specify a custom output file for the lock
- `pip lock package` : lock a specific package and its dependencies

## Categories
packages, files

## Links
- More information: https://pip.pypa.io/en/stable/cli/pip_lock/
- tldr page: https://tldr.inbrowser.app/pages/common/pip-lock
- Wikipedia: https://en.wikipedia.org/wiki/Pip_(package_manager)
