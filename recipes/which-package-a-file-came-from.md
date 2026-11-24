# which package a file came from

Find the package that installed a file or a command, or the one that
would install a file you do not have yet.

## Steps
- `command -v ls` : the file a command runs, to ask about
- `dpkg -S /usr/bin/ls` : the package that installed it (Debian, Ubuntu)
- `rpm -qf /usr/bin/ls` : the same on Fedora, RHEL and openSUSE
- `pacman -Qo /usr/bin/ls` : the same on Arch
- `apt-file search bin/ncdu` : the package that would install a file not here yet (Debian, Ubuntu; needs apt-file)
- `dnf provides '*/bin/ncdu'` : the same on Fedora and RHEL

## Notes
- Every page in lore shows its command's package under VERSION, found the same way.
- macOS: Homebrew's commands are links into its folder: `ls -l "$(command -v rg)"` shows the formula.

## See also
- dpkg
- rpm
- pacman
- apt-file

## Categories
packages files
