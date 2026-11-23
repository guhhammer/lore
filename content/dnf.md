# dnf

Package manager for Fedora 41+ and RHEL 10. For equivalent commands in other package managers, see <https://wiki.archlinux.org/title/Pacman/Rosetta>.

## Usage
- `sudo dnf up` : upgrade installed packages to the newest available versions
- `dnf se keyword1 keyword2 ...` : search packages via keywords
- `dnf if package` : display details about a package
- `sudo dnf in package1 package2 ...` : install new packages (use `--assumeyes` to confirm all prompts automatically)
- `sudo dnf rm package1 package2 ...` : remove packages
- `dnf ls --installed` : list installed packages
- `dnf provides command` : find which packages provide a given command
- `sudo dnf clean all|dbcache|expire-cache|metadata|packages` : clean cached data

## Categories
packages

## Links
- More information: https://dnf5.readthedocs.io/en/latest/commands/index.html
- tldr page: https://tldr.inbrowser.app/pages/linux/dnf
