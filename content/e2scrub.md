# e2scrub

Check the contents of a mounted ext[234] file system.

## Usage
- `e2scrub [OPTION] MOUNTPOINT | DEVICE` : the general form, as the manual gives it
- `e2scrub -n` : print what commands e2scrub would execute to check the file system
- `e2scrub -r` : remove the e2scrub snapshot and exit without checking anything
- `e2scrub -t` : run fstrim(1) on the mounted file system if no errors are found
- `e2scrub -V` : print version information and exit

## Categories
disks, files
