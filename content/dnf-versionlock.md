# dnf versionlock

Protect packages from updates to newer versions. Not default to `dnf` but supported via `dnf-plugins-core`.

## Usage
- `dnf versionlock` : list the current versionlock entries
- `dnf versionlock add package` : add a versionlock for all available packages matching the spec
- `dnf versionlock exclude package` : add an exclude (within versionlock) for the available packages matching the spec
- `dnf versionlock delete package` : remove any matching versionlock entries
- `dnf versionlock clear` : remove all versionlock entries

## Categories
packages

## Links
- More information: https://dnf-plugins-core.readthedocs.io/en/latest/versionlock.html
- tldr page: https://tldr.inbrowser.app/pages/linux/dnf-versionlock
