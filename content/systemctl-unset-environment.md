# systemctl unset-environment

Unset one or more service manager environment variables. This undoes the effect of `systemctl set-environment`.

## Usage
- `systemctl unset-environment var` : unset a single environment variable
- `systemctl unset-environment var1 var2 ...` : unset multiple environment variables at once
- `systemctl unset-environment var --user` : unset an environment variable in the user service manager

## Categories
system

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/systemctl-unset-environment
