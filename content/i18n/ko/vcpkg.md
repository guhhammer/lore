# vcpkg

C/C++ 라이브러리를 위한 패키지 관리자. 참고: 패키지는 시스템에 설치되지 않습니다. 사용하려면 빌드 시스템(예: CMake)에 `vcpkg`를 사용하도록 지정해야 합니다.

## Usage
- `vcpkg install curl` : `vcpkg` 환경에 `libcurl` 패키지를 빌드하고 추가
- `vcpkg install --triplet=wasm32-emscripten zlib` : `emscripten` 도구 체인을 사용하여 `zlib`를 빌드하고 추가
- `vcpkg search 패키지_이름` : 패키지 검색
- `cmake -B build -DCMAKE_TOOLCHAIN_FILE=경로/대상/vcpkg_설치_디렉토리/scripts/buildsystems/vcpkg.cmake` : CMake 프로젝트를 `vcpkg` 패키지를 사용하도록 설정

## Links
- More information: https://learn.microsoft.com/vcpkg/
