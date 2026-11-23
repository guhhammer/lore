# rpmspec

Query an RPM spec file.

## Usage
- `rpmspec --query path/to/rpm.spec` : list binary packages which would be generated from an RPM spec file
- `rpmspec --querytags` : list all options for `--queryformat`
- `rpmspec --query --queryformat "%{name}: %{summary}\n" path/to/rpm.spec` : get summary information for single binary packages generated from an RPM spec file
- `rpmspec --query --srpm path/to/rpm.spec` : get the source package which would be generated from an RPM spec file
- `rpmspec --parse path/to/rpm.spec` : parse an RPM spec file to `stdout`

## Categories
files, databases, packages

## Links
- More information: https://manned.org/rpmspec
- tldr page: https://tldr.inbrowser.app/pages/linux/rpmspec
