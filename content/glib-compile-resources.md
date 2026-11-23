# glib-compile-resources

Compile resource files (e.g. images) into a binary resource bundle. These may be linked into GTK applications using the GResource API.

## Usage
- `glib-compile-resources file.gresource.xml` : compile resources referenced in `file.gresource.xml` to a `.gresource` binary
- `glib-compile-resources --generate-source file.gresource.xml` : compile resources referenced in `file.gresource.xml` to a C source file
- `glib-compile-resources --generate --target=file.ext file.gresource.xml` : compile resources in `file.gresource.xml` to a chosen target file, with `.c`, `.h`, or `.gresource` extension
- `glib-compile-resources --generate-dependencies file.gresource.xml` : print a list of resource files referenced in `file.gresource.xml`

## Categories
development, files, text

## Links
- More information: https://manned.org/glib-compile-resources
- tldr page: https://tldr.inbrowser.app/pages/common/glib-compile-resources
