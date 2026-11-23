# arduino

Arduino Studio - Integrated Development Environment for the Arduino platform.

## Usage
- `arduino --verify path/to/file.ino` : build a sketch
- `arduino --upload path/to/file.ino` : build and upload a sketch
- `arduino --board arduino:avr:nano:cpu=atmega328p --port /dev/ttyACM0 --upload path/to/file.ino` : build and upload a sketch to an Arduino Nano with an Atmega328p CPU, connected on port `/dev/ttyACM0`
- `arduino --pref name=value` : set the preference `name` to a given `value`
- `arduino --pref build.path=path/to/build_directory --verify path/to/file.ino` : build a sketch, put the build results in the build directory, and reuse any previous build results in that directory
- `arduino --save-prefs` : save any (changed) preferences to `preferences.txt`
- `arduino --install-boards "arduino:sam"` : install the latest SAM board
- `arduino --install-library "Bridge:1.0.0,Servo:1.2.0"` : install Bridge and Servo libraries

## Categories
files, development, network

## Links
- More information: https://github.com/arduino/Arduino/blob/master/build/shared/manpage.adoc
- tldr page: https://tldr.inbrowser.app/pages/common/arduino
