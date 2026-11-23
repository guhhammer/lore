# vswhere

Localiza instalaciones de Visual Studio 2017 y posteriores.

## Usage
- `vswhere -products * -latest -prerelease -find **\VC\Auxiliary\Build\vcvarsall.bat` : encuentra la ruta de vcvarsall.bat para configurar variables de entorno
- `vswhere -products * -latest -prerelease -find **\Hostx64\x64\*` : encuentra el directorio del compilador x64 MSVC (`cl.exe`, etc.)
- `vswhere -products * -latest -prerelease -find **\Llvm\bin\*` : encuentra el directorio de Clang incluido con Visual Studio (clang-cl, clang-tidy, etc.)
- `vswhere -products * -latest -prerelease -find MSBuild\**\Bin\MSBuild.exe` : encuentra la ruta de `MSBuild.exe`

## Links
- More information: https://github.com/microsoft/vswhere
