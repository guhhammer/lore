# pio pkg

Manage packages in the registry. Packages can only be removed within 72 hours (3 days) from the date that they are published.

## Usage
- `pio pkg pack -o path/to/package.tar.gz` : create a package tarball from the current directory
- `pio pkg publish` : create and publish a package tarball from the current directory
- `pio pkg publish --private` : publish the current directory and restrict public access to it
- `pio pkg publish path/to/package.tar.gz` : publish a package
- `pio pkg publish path/to/package.tar.gz --released-at "2021-04-08 21:15:38"` : publish a package with a custom release date (UTC)
- `pio pkg unpublish package` : remove all versions of a published package from the registry
- `pio pkg unpublish package@version` : remove a specific version of a published package from the registry
- `pio pkg unpublish --undo package@version` : undo the removal, putting all versions or a specific version of the package back into the registry

## Categories
packages, files, archives

## Links
- More information: https://docs.platformio.org/en/latest/core/userguide/package/
- tldr page: https://tldr.inbrowser.app/pages/common/pio-pkg
