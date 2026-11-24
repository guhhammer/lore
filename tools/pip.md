# pip

pip installs Python packages from PyPI. Use it inside a virtual environment.

## Tasks
### Install
- `pip install requests` : install a package
- `pip install requests==2.32.3` : an exact version
- `pip install 'requests~=2.30'` : any compatible 2.x from 2.30 up
- `pip install -U requests` : upgrade to the newest version
- `pip install -r requirements.txt` : install every package in a file
- `pip install -e .` : install the current folder as editable (changes show at once)
- `pip install ./dist/NAME.whl` : install from a local file
- `pip install git+https://github.com/USER/REPO.git` : install straight from a git repository
### Look
- `pip list` : everything installed
- `pip list --outdated` : packages with a newer version
- `pip show requests` : details, location and what it needs
- `pip freeze` : installed packages as pinned lines
- `pip index versions requests` : every version PyPI has
- `pip check` : find broken dependencies
### Remove
- `pip uninstall requests` : remove a package
- `pip uninstall -r requirements.txt` : remove every package in a file
- `pip cache purge` : empty the download cache
### Save
- `pip freeze > requirements.txt` : write the installed versions to a file
- `pip download requests -d DIR` : download without installing
- `pip wheel . -w dist` : build a wheel of the current project
### Fix problems
- `python3 -m pip install requests` : use the pip of one exact python
- `pip install --user requests` : install for your user only, outside a venv
- `pip install --no-cache-dir requests` : ignore the cache
- `pip install --break-system-packages requests` : override the system-managed protection (can break the OS tools)
- `pip config list` : show the pip settings in use

## Notes
- Activate a virtual environment first, or packages land in the wrong place.
- `python3 -m pip` makes sure pip belongs to the python you mean.
- Debian, Ubuntu and Fedora mark the system python as externally managed: use a venv, pipx or uv instead of `--break-system-packages`.
- Pin versions in `requirements.txt` for apps; libraries should state ranges.

## See also
- python-venv
- uv
- pipx
- python3

## Links
- Manual: https://pip.pypa.io/en/stable/cli/
- Packages: https://pypi.org

## Categories
packages development
