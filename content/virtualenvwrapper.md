# virtualenvwrapper

Group of simple wrapper commands for Python's `virtualenv` tool.

## Usage
- `mkvirtualenv virtualenv_name` : create a new Python `virtualenv` in `$WORKON_HOME`
- `mkvirtualenv --python /usr/local/bin/python3.8 virtualenv_name` : create a `virtualenv` for a specific Python version
- `workon virtualenv_name` : activate or use a different `virtualenv`
- `deactivate` : stop the `virtualenv`
- `lsvirtualenv` : list all virtual environments
- `rmvirtualenv virtualenv_name` : remove a `virtualenv`
- `virtualenvwrapper` : get summary of all virtualenvwrapper commands

## Categories
users

## Links
- More information: https://virtualenvwrapper.readthedocs.io/en/latest/command_ref.html
- tldr page: https://tldr.inbrowser.app/pages/common/virtualenvwrapper
