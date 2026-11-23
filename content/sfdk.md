# sfdk

Frontend of the Sailfish SDK. Some subcommands such as `init`, `build-init`, `device`, etc. have their own usage documentation.

## Usage
- `sfdk config target=SailfishOS-5.0.0.62-aarch64` : set up the current environment for building for SailfishOS with a specific version and architecture target
- `sfdk build-init` : initialize the current directory as the build directory
- `sfdk -C path/to/project build` : execute build steps of the RPM SPEC file for a specific project
- `sfdk -c 'target=SailfishOS-5.0.0.62-armv7hl' build-shell --maintain ssu lr` : list repositories in the SailfishOS 5.0.0.62 armv7hl build target
- `sfdk config device="Sailfish OS Emulator 5.0.0.62"; sfdk deploy --sdk` : deploy the package to the emulator
- `sfdk --help` : display help
- `sfdk --help-building|testing|maintaining|ide|all` : display help for specific topic
- `sfdk --version` : display version

## Categories
development, files, hardware

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/sfdk
