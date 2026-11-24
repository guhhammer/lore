# add a user

Create an account with a home folder and a password, and let it use sudo
if it should.

## Steps
- `sudo adduser NAME` : create the user, its home folder and its password, asking for each (Debian, Ubuntu)
- `sudo useradd -m -s /bin/bash NAME` : the same on any Linux: a home folder (-m) and bash as its shell
- `sudo passwd NAME` : set its password (needed after useradd)
- `sudo usermod -aG sudo NAME` : let it use sudo; the group is `wheel` on Fedora, RHEL and Arch
- `id NAME` : check its groups
- `sudo -iu NAME` : open a shell as that user, to try it

## Notes
- Keep the `-a` in `usermod -aG`: without it, the user is taken out of every other group.
- A new group takes effect at the user's next login.
- macOS: System Settings, Users & Groups, or `sysadminctl -addUser` (see its manual).

## See also
- adduser
- useradd
- usermod
- passwd

## Categories
users system
