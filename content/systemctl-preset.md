# systemctl preset

Reset the enablement state of unit files to the defaults specified in preset policy files.

## Usage
- `systemctl preset unit1 unit2 ...` : reset the enablement state to preset defaults
- `systemctl preset unit --preset-mode enable-only` : enable only if marked as enabled in the preset policy
- `systemctl preset unit --preset-mode disable-only` : disable only if marked as disabled in the preset policy
- `systemctl preset unit -q` : suppress output and return only the exit code

## See also
- systemctl preset-all
- systemctl list-unit-files

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/systemctl-preset
