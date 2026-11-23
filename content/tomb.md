# tomb

Manage encrypted storage directories that can be safely transported and hidden in a filesystem.

## Usage
- `tomb dig -s 100 encrypted_directory.tomb` : create a new tomb with an initial size of 100 MB
- `tomb forge encrypted_directory.tomb.key` : create a new key file that can be used to lock a tomb; user will be prompted for a password for the key
- `tomb forge encrypted_directory.tomb.key -f` : forcefully create a new key, even if the tomb isn't allowing key forging (due to swap)
- `tomb lock encrypted_directory.tomb -k encrypted_directory.tomb.key` : initialize and lock an empty tomb using a key made with `forge`
- `tomb open encrypted_directory.tomb -k encrypted_directory.tomb.key` : mount a tomb (by default in `/media`) using its key, making it usable as a regular filesystem directory
- `tomb close encrypted_directory.tomb` : close a tomb (fails if the tomb is being used by a process)
- `tomb slam all` : forcefully close all open tombs, killing any applications using them
- `tomb list` : list all open tombs

## Categories
disks, files

## Links
- More information: https://dyne.org/docs/tomb/manpage/
- tldr page: https://tldr.inbrowser.app/pages/linux/tomb
