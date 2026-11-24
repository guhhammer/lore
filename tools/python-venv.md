# python venv

A virtual environment is a private folder of Python packages for one project, so projects do not break each other. This page covers making, using and rebuilding one.

## Tasks
### Make one
- `python3 -m venv .venv` : make an environment in the folder .venv
- `python3 -m venv --system-site-packages .venv` : also see the system's packages
- `python3 -m venv --clear .venv` : empty an existing environment and start again
- `sudo apt install python3-venv` : add the module when it is missing (Debian, Ubuntu)
### Use it
- `source .venv/bin/activate` : turn it on (bash, zsh)
- `source .venv/bin/activate.fish` : turn it on (fish)
- `deactivate` : turn it off
- `.venv/bin/python script.py` : run with it without activating
- `which python` : check that the active python is the one in .venv
- `python -m pip install --upgrade pip` : update pip inside it
### Packages
- `pip install requests` : install a package into the active environment
- `pip install -r requirements.txt` : install everything a project lists
- `pip freeze > requirements.txt` : write the installed versions to a file
- `pip list` : show what is installed
- `pip list --outdated` : show what has a newer version
- `pip uninstall requests` : remove a package
### Share and rebuild
- `rm -rf .venv && python3 -m venv .venv` : throw the environment away and make a new one (deletes .venv)
- `pip install -e .` : install the current project in editable mode
- `pip install -r requirements.txt -c constraints.txt` : install with pinned versions
- `python3 -m venv ~/.venvs/NAME` : keep an environment outside the project
- `python -m pip check` : look for broken dependencies
### Tools in their own environment
- `pipx install black` : install a command line tool in its own environment
- `pipx upgrade-all` : update every pipx tool

## Notes
- Never commit `.venv`: add it to `.gitignore` and commit `requirements.txt` instead.
- A venv holds a path: moving or renaming the folder breaks it, so make a new one.
- Activation only changes your current shell; closing the terminal turns it off.
- On Debian and Ubuntu a bare `pip install` outside a venv is refused (externally managed): use a venv or pipx.

## See also
- pip
- uv
- python3
- pipx

## Links
- venv: https://docs.python.org/3/library/venv.html
- Tutorial: https://docs.python.org/3/tutorial/venv.html

## Categories
development
