# vf

VirtualFish is a fish shell tool for managing Python virtual environments.

## Usage
- `vf new virtualenv_name` : create a virtual environment
- `vf new --python /usr/local/bin/python3.8 virtualenv_name` : create a virtual environment for a specific Python version
- `vf activate virtualenv_name` : activate and use the specified virtual environment
- `vf connect` : connect the current virtualenv to the current directory, so that it is activated automatically as soon as you enter it (and deactivated as soon as you leave)
- `vf deactivate` : deactivate the current virtual environment
- `vf ls` : list all virtual environments
- `vf rm virtualenv_name` : remove a virtual environment
- `vf help` : display help

## Categories
shell

## Links
- More information: https://virtualfish.readthedocs.io/en/latest/
- tldr page: https://tldr.inbrowser.app/pages/common/vf
