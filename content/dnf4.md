# dnf4

Package manager for RHEL 8/9 and older Fedora versions (pre-41). For equivalent commands in other package managers, see <https://wiki.archlinux.org/title/Pacman/Rosetta>.

## Usage
- `sudo dnf4 up` : upgrade installed packages to the newest available versions
- `dnf4 se keyword1 keyword2 ...` : search packages via keywords
- `dnf4 if package` : display details about a package
- `sudo dnf4 in package1 package2 ...` : install a new package (use `--assumeyes` to confirm all prompts automatically)
- `sudo dnf4 rm package1 package2 ...` : remove a package
- `dnf4 ls --installed` : list installed packages
- `dnf4 wp command` : find which packages provide a given command
- `dnf4 hist` : view all past operations

## Categories
packages, network, users

## Links
- More information: https://dnf.readthedocs.io/en/latest/command_ref.html
- tldr page: https://tldr.inbrowser.app/pages/linux/dnf4
