# bun pm pack

Create a `.tgz` archive containing the files that would be published to npm (same behavior as `npm pack`).

## Usage
- `bun pm pack` : create a tarball from the current workspace
- `bun pm pack --dry-run` : simulate packing, running all steps without writing the tarball to disk
- `bun pm pack --destination path/to/directory` : save the tarball to a specific directory
- `bun pm pack --filename filename` : set an exact filename for the tarball
- `bun pm pack --ignore-scripts` : skip prepack, postpack, and prepare scripts
- `bun pm pack --gzip-level 5` : set the `gzip` compression level (0-9, default: 9)
- `bun pm pack --quiet` : output only the tarball filename and suppress verbose logs

## Categories
archives, files

## Links
- More information: https://bun.com/docs/pm/cli/pm#pack
- tldr page: https://tldr.inbrowser.app/pages/common/bun-pm-pack
