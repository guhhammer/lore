# pyinfra

Automates infrastructure at a large scale.

## Usage
- `pyinfra target_ip_address exec -- command_name_and_arguments` : execute a command over SSH
- `pyinfra path/to/target_list.py path/to/deploy.py` : execute contents of a deploy file on a list of targets
- `pyinfra @local path/to/deploy.py` : execute commands on locally
- `pyinfra @docker/container path/to/deploy.py` : execute commands over Docker

## Categories
files, containers, network

## Links
- More information: https://docs.pyinfra.com/en/3.x/cli.html
- tldr page: https://tldr.inbrowser.app/pages/common/pyinfra
