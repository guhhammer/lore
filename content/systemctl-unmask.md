# systemctl unmask

Unmask units to make them startable again. This undoes the effect of `systemctl mask`.

## Usage
- `systemctl unmask service_name` : unmask a service
- `systemctl unmask service_name --now` : unmask and start a service immediately
- `systemctl unmask service_name --user` : unmask a user service

## Categories
system

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/systemctl-unmask
