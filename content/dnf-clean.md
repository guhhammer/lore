# dnf clean

Perform cleanup of temporary files kept for Red Hat-based repositories.

## Usage
- `dnf clean dbcache` : remove cache files generated from the repository metadata
- `dnf clean expire-cache` : mark the repository metadata as expired
- `dnf clean metadata` : remove repository metadata
- `dnf clean packages` : remove any cached packages from the system
- `dnf clean all` : clean up all DNF repository specific metadata and cached files (all of the above)

## Categories
packages

## Links
- More information: https://dnf.readthedocs.io/en/latest/command_ref.html#clean-command
- tldr page: https://tldr.inbrowser.app/pages/linux/dnf-clean
