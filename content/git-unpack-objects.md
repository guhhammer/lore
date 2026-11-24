# git unpack-objects

Git command for unpacking objects from a packed archive.

## Usage
- `git <pack-file.pack unpack-objects -n` : simulate the pack file without unpacking objects
- `git <pack-file.pack unpack-objects -q` : suppress percentage progress typically shown by the command
- `git <pack-file.pack unpack-objects -r` : unpack a corrupt packfile beyond the first corruption
- `git <pack-file.pack unpack-objects --strict` : don't write objects containing broken content or links
- `git <pack-file.pack unpack-objects --max-input-size=size` : die, given a pack larger than `size`

## Categories
version-control, files, archives

## Links
- More information: https://git-scm.com/docs/git-unpack-objects
- tldr page: https://tldr.inbrowser.app/pages/common/git-unpack-objects
- Wikipedia: https://en.wikipedia.org/wiki/Git
