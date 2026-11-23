# schroot

Run a command or start an interactive shell with a different root directory. More customizable than `chroot`.

## Usage
- `schroot --list` : list available chroots
- `schroot --chroot chroot command` : run a command in a specific chroot
- `schroot --chroot chroot command -- command_options` : run a command with options in a specific chroot
- `schroot --all command` : run a command in all available chroots
- `schroot --chroot chroot --user user` : start an interactive shell within a specific chroot as a specific user
- `schroot --begin-session --chroot chroot` : begin a new session (a unique session ID is returned on `stdout`)
- `schroot --run-session --chroot session_id` : connect to an existing session
- `schroot --end-session --chroot session_id` : end an existing session

## Categories
shell, files, users

## Links
- More information: https://wiki.debian.org/Schroot
- tldr page: https://tldr.inbrowser.app/pages/linux/schroot
