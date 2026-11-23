# eim

Install and manage ESP-IDF.

## Usage
- `eim install` : install the default (latest) ESP-IDF version in the default location (`C:\esp` on Windows and `~/.espressif` on POSIX systems)
- `eim install -i v5.3.2` : install a specific ESP-IDF version
- `eim wizard` : run the interactive, guided installation wizard
- `eim install -i v5.3.2 -p /opt/esp-idf -n false` : install a specific version to a custom path, forcing interactive mode (to prompt for choices)
- `eim list` : list all currently installed ESP-IDF versions
- `eim remove v5.3.2` : remove a specific installed ESP-IDF version
- `eim install -c path/to/config.toml` : install in headless mode using all options defined in a TOML configuration file
- `eim install --use-local-archive path/to/archive.zst` : install offline using a pre-downloaded archive file

## Categories
packages, files, archives

## Links
- More information: https://docs.espressif.com/projects/idf-im-ui/en/latest/cli_commands.html
- tldr page: https://tldr.inbrowser.app/pages/common/eim
