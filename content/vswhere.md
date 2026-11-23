# vswhere

Locate Visual Studio 2017 and newer installations.

## Usage
- `vswhere -products * -latest -prerelease -find **\VC\Auxiliary\Build\vcvarsall.bat` : find the path of `vcvarsall.bat` to set environment variables
- `vswhere -products * -latest -prerelease -find **\Hostx64\x64\*` : find the directory of the x64 MSVC compiler (`cl.exe`, etc.)
- `vswhere -products * -latest -prerelease -find **\Llvm\bin\*` : find the directory of Clang bundled with Visual Studio bundled (clang-cl, clang-tidy, etc.)
- `vswhere -products * -latest -prerelease -find MSBuild\**\Bin\MSBuild.exe` : find the path of `MSBuild.exe`

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
files, windows

## Links
- More information: https://github.com/microsoft/vswhere
- tldr page: https://tldr.inbrowser.app/pages/windows/vswhere
