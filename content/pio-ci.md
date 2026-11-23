# pio ci

Build PlatformIO projects with an arbitrary source code structure. This will create a new temporary project which the source code will be copied into.

## Usage
- `pio ci path/to/project` : build a PlatformIO project in the default system temporary directory and delete it afterwards
- `pio ci -l path/to/library_directory path/to/project` : build a PlatformIO project and specify specific libraries
- `pio ci -b board path/to/project` : build a PlatformIO project and specify a specific board (`pio boards` lists all of them)
- `pio ci --build-dir path/to/build_directory path/to/project` : build a PlatformIO project in a specific directory
- `pio ci --keep-build-dir path/to/project` : build a PlatformIO project and don't delete the build directory
- `pio ci -c path/to/platformio.ini` : build a PlatformIO project using a specific configuration file

## Categories
development, files

## Links
- More information: https://docs.platformio.org/en/latest/core/userguide/cmd_ci.html
- tldr page: https://tldr.inbrowser.app/pages/common/pio-ci
