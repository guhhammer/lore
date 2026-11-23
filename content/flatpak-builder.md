# flatpak-builder

Help build application dependencies.

## Usage
- `flatpak-builder path/to/build_directory path/to/manifest` : build a Flatpak and export it to a new repository
- `flatpak-builder --repo repository_name path/to/build_directory path/to/manifest` : build a Flatpak and export it to the specified repository
- `flatpak-builder --install path/to/build_directory path/to/manifest` : build a Flatpak and install it locally
- `flatpak-builder --gpg-sign key_id --repo repository_name path/to/manifest` : build and sign a Flatpak and export it to the specified repository
- `flatpak-builder --run path/to/build_directory path/to/manifest sh` : run a shell inside of an application sandbox without installing it

## Categories
files, packages, development

## Links
- More information: https://docs.flatpak.org/en/latest/flatpak-builder-command-reference.html
- tldr page: https://tldr.inbrowser.app/pages/linux/flatpak-builder
