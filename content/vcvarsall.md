# vcvarsall

Set up the environment variables required for using the Microsoft Visual Studio tools. The path of `vcvarsall` for a certain Visual Studio installation can be found using `vswhere`.

## Usage
- `vcvarsall x64` : set up the environment for native x64
- `vcvarsall x64_x86` : set up the environment for cross-compiled native x86 from the x64 host
- `vcvarsall x64_arm64` : set up the environment for cross-compiled native Arm x64 from the x64 host
- `vcvarsall x64 uwp` : set up the environment for native UWP x64

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
network, files, windows

## Links
- More information: https://learn.microsoft.com/cpp/build/building-on-the-command-line
- tldr page: https://tldr.inbrowser.app/pages/windows/vcvarsall
