# urpmi

Install packages in Mageia.

## Usage
- `sudo urpmi package|path/to/file.rpm` : install a package from the repository or from a local RPM file
- `urpmi --no-install package` : download a package without installing it
- `sudo urpmi --auto-select` : update all installed packages (run `urpmi.update -a` to get the available updates)
- `sudo urpmi --parallel local package` : update a package of one or more machines on the network according to `/etc/urpmi/parallel.cfg`
- `sudo urpmi $(urpmq --auto-orphans -f)` : mark all orphaned packages as manually installed

## See also
- urpmi.update
- urpme
- urpmi.addmedia
- urpmi.removemedia
- urpmf
- urpmq

## Categories
packages, network, users

## Links
- More information: https://man.linuxreviews.org/man8/urpmi.8.html
- tldr page: https://tldr.inbrowser.app/pages/linux/urpmi
- Wikipedia: https://en.wikipedia.org/wiki/Urpmi
