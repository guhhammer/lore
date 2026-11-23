# komac

Create WinGet manifests for the `winget-pkgs` repository.

## Usage
- `komac new Package.Identifier -v 1.2.3 -u https://example.com/app.exe` : create a new package from scratch
- `komac update Package.Identifier -v 1.2.3 -u https://example.com/app.exe` : update an existing package with a new version
- `komac update Package.Identifier -v 1.2.3 -u https://example.com/app.exe https://example.com/app.msi ... -s` : update a package with multiple URLs and automatically submit
- `komac remove Package.Identifier -v 1.2.3` : remove a version from winget-pkgs
- `komac list Package.Identifier` : list all versions for a package
- `komac sync` : sync your fork of winget-pkgs with the upstream repository
- `komac token add -t your_github_token` : update the stored GitHub token
- `komac complete bash|elvish|fish|powershell|zsh` : generate shell autocompletion script

## Categories
packages, network

## Links
- More information: https://github.com/russellbanks/Komac
- tldr page: https://tldr.inbrowser.app/pages/common/komac
