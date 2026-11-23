# bwrap

Run programs in a lightweight sandbox.

## Usage
- `bwrap --ro-bind / / /bin/bash` : run a program in a read-only environment
- `bwrap --dev-bind /dev /dev --proc /proc --ro-bind / / --tmpfs /tmp /bin/bash` : give the environment access to devices, process information, and create a `tmpfs` for it

## Links
- More information: https://manned.org/bwrap
- tldr page: https://tldr.inbrowser.app/pages/linux/bwrap
