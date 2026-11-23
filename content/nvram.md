# nvram

Manipulate firmware variables.

## Usage
- `nvram -p` : print all the variables stored in the NVRAM
- `nvram -xp` : print all the variables stored in the NVRAM using xML format
- `sudo nvram name="value"` : modify the value of a firmware variable
- `sudo nvram -d name` : delete a firmware variable
- `sudo nvram -c` : clear all the firmware variables
- `sudo nvram -xf path/to/file.xml` : set a firmware variable from a specific xML file

## Categories
hardware, users, files, macos

## Links
- More information: https://keith.github.io/xcode-man-pages/nvram.8.html
- tldr page: https://tldr.inbrowser.app/pages/osx/nvram
