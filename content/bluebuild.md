# bluebuild

Build Containerfiles and custom images based on your `recipe.yml`.

## Usage
- `bluebuild build path/to/recipe.yml` : build a recipe
- `bluebuild validate path/to/recipe.yml` : validate a recipe
- `bluebuild generate -o Containerfile path/to/recipe.yml` : generate a Containerfile
- `bluebuild generate-iso --output-dir path/to/output_directory --iso-name iso_name.iso recipe path/to/recipe.yml` : generate an ISO from a recipe
- `bluebuild -h` : display help

## Categories
development, files, containers

## Links
- More information: https://github.com/blue-build/cli#how-to-use
- tldr page: https://tldr.inbrowser.app/pages/linux/bluebuild
