# rpmbuild

RPM Package Build tool.

## Usage
- `rpmbuild -ba path/to/spec_file` : build binary and source packages
- `rpmbuild -bb path/to/spec_file` : build a binary package without source package
- `rpmbuild -bb path/to/spec_file --define "variable1 value1" --define "variable2 value2"` : specify additional variables when building a package

## Categories
packages, development, files

## Links
- More information: https://manned.org/rpmbuild
- tldr page: https://tldr.inbrowser.app/pages/linux/rpmbuild
