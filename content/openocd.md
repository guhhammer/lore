# openocd

Debug and program embedded systems with OpenOCD.

## Usage
- `openocd -f config_file.cfg` : attach OpenOCD session to a board with a configuration file
- `openocd -f config_file1.cfg -f config_file2.cfg` : attach OpenOCD session to a board with multiple configuration files
- `openocd -f config_file.cfg -c "command"` : attach OpenOCD session to a board with configuration files and a list of commands to be executed on server startup
- `openocd -s path/to/search -f config_file.cfg` : use configuration files in the specified path
- `target extended-remote localhost` : [Interactive] After OpenOCD startup, connect GDB to OpenOCD default port 3333
- `ls /usr/local/share/openocd/scripts` : list site-wide script library

## Categories
development, files, network

## Links
- More information: https://manned.org/openocd
- tldr page: https://tldr.inbrowser.app/pages/common/openocd
