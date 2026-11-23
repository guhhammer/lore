# dpkg-scanpackages

Create Packages index files.

## Usage
- `dpkg-scanpackages [option...] binary-path [override-file [path-prefix]] > Packages` : the general form, as the manual gives it
- `dpkg-scanpackages -t` : scan for *.type packages, instead of *.deb
- `dpkg-scanpackages -m` : include all found packages in the output
- `dpkg-scanpackages --version` : show the version and exit

## Categories
packages, files
