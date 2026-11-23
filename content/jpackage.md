# jpackage

Packages a Java application, with its runtime, into a native installer or app image.

## Usage
- `jpackage --name App --input lib --main-jar app.jar --main-class Main` : build an installer for the current system
- `jpackage --type app-image --name App --input lib --main-jar app.jar` : build a plain app folder, no installer
- `jpackage --type deb --name App --input lib --main-jar app.jar` : build a Debian package

## Categories
containers, media, packages
