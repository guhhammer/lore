# arduino-builder

Compile arduino sketches. Note: This command is deprecated, use `arduino` instead.

## Usage
- `arduino-builder -compile path/to/sketch.ino` : compile a sketch
- `arduino-builder -debug-level 1..10` : specify the debug level (default: 5)
- `arduino-builder -build-path path/to/build_directory` : specify a custom build directory
- `arduino-builder -build-options-file path/to/build.options.json` : use a build option file, instead of specifying `-hardware`, `-tools`, etc. manually every time
- `arduino-builder -verbose true` : enable verbose mode

## Categories
development, files

## Links
- More information: https://github.com/arduino/arduino-builder
- tldr page: https://tldr.inbrowser.app/pages/common/arduino-builder
