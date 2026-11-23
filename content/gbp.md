# gbp

A system to integrate the Debian package build system with Git.

## Usage
- `gbp import-dsc path/to/package.dsc` : convert an existing Debian package to gbp
- `gbp buildpackage -jauto -us -uc` : build the package in the current directory using the default builder (`debuild`)
- `DIST=bullseye ARCH=amd64 gbp buildpackage -jauto -us -uc --git-builder=git-pbuilder` : build a package in a `pbuilder` environment for Debian Bullseye
- `gbp buildpackage -jauto -us -uc --changes-options=-S` : specify a package to be a source-only upload in the `.changes` file (see <https://wiki.debian.org/SourceOnlyUpload>)
- `gbp import-orig --pristine-tar path/to/package.tar.gz` : import a new upstream release

## Categories
packages, version-control, development

## Links
- More information: https://honk.sigxcpu.org/projects/git-buildpackage/manual-html/gbp.html
- tldr page: https://tldr.inbrowser.app/pages/linux/gbp
