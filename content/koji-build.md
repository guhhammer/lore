# koji build

Build an RPM package.

## Usage
- `koji build target path/to/src.rpm` : build a package from `src.rpm`
- `koji build target git+https://src.fedoraproject.org/rpms/vim.git#e847a50297a216229050bf4db3d06a139104e7cf` : build a package from an SCM (Source Code Management) URL
- `koji build target path/to/src.rpm --scratch` : perform a scratch build
- `koji build target path/to/src.rpm --wait` : wait on the build, even if it's running in the background
- `koji build target path/to/src.rpm --nowait` : don't wait on build
- `koji build -h` : display help

## Categories
development, packages, files

## Links
- More information: https://docs.pagure.org/koji/
- tldr page: https://tldr.inbrowser.app/pages/linux/koji-build
