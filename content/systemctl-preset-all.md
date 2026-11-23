# systemctl preset-all

Reset the enablement state of all installed units to the defaults specified in preset policy files.

## Usage
- `sudo systemctl preset-all` : reset the enablement state of all installed units
- `sudo systemctl preset-all --preset-mode enable-only` : enable only if marked as enabled in the preset policy
- `sudo systemctl preset-all --preset-mode disable-only` : disable only if marked as disabled in the preset policy
- `sudo systemctl preset-all -q` : suppress output and return only the exit code

## See also
- systemctl preset
- systemctl list-unit-files

## Categories
users

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/systemctl-preset-all
