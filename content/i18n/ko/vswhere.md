# vswhere

Visual Studio 2017 및 더 최신 설치를 찾습니다.

## Usage
- `vswhere -products * -latest -prerelease -find **\VC\Auxiliary\Build\vcvarsall.bat` : vcvarsall.bat의 경로를 찾아 환경 변수를 설정
- `vswhere -products * -latest -prerelease -find **\Hostx64\x64\*` : x64 MSVC 컴파일러 (`cl.exe` 등)의 디렉토리 찾기
- `vswhere -products * -latest -prerelease -find **\Llvm\bin\*` : visual Studio에 포함된 Clang (clang-cl, clang-tidy 등)의 디렉토리 찾기
- `vswhere -products * -latest -prerelease -find MSBuild\**\Bin\MSBuild.exe` : `MSBuild.exe`의 경로 찾기

## Links
- More information: https://github.com/microsoft/vswhere
