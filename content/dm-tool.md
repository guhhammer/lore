# dm-tool

A tool to communicate with the display manager.

## Usage
- `dm-tool switch-to-greeter` : show the greeter while keeping current desktop session open and waiting to be restored upon authentication by logged in user
- `dm-tool lock` : lock the current session
- `dm-tool switch-to-user username session` : switch to a specific user, showing an authentication prompt if required
- `dm-tool add-seat xlocal name=value` : add a dynamic seat from within a running LightDM session

## Categories
desktop, users

## Links
- More information: https://manned.org/dm-tool
- tldr page: https://tldr.inbrowser.app/pages/linux/dm-tool
