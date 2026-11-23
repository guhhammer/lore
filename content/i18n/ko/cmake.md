# cmake

네이티브 빌드 시스템을 위한 빌드 설정 파일을 생성하는, 크로스 플랫폼 빌드 자동화 도구.

## Usage
- `cmake 경로/대상/프로젝트_디렉터리` : 프로젝트 디렉터리의 `CMakeLists.txt`를 사용해 현재 디렉터리에 빌드 설정을 생성
- `cmake --build 경로/대상/빌드_디렉터리` : 지정한 디렉터리의 빌드 설정을 사용해 빌드 산출물을 생성
- `cmake --install 경로/대상/빌드_디렉터리 --strip` : 빌드 산출물을 `/usr/local/`에 설치하고 디버그 심볼을 제거
- `cmake 경로/대상/프로젝트_디렉터리 -D CMAKE_BUILD_TYPE=Release` : CMake 변수를 사용해 빌드 타입을 `Release`로 설정한 빌드 설정을 생성
- `cmake -G generator_name 경로/대상/프로젝트_디렉터리` : `generator_name` 기반 빌드 시스템으로 사용해 빌드 설정을 생성
- `cmake --install 경로/대상/빌드_디렉터리 --strip --prefix 경로/대상/디렉터리` : 사용자 지정 경로 접두사를 사용해 빌드 산출물을 설치
- `cmake --build 경로/대상/빌드_디렉터리 -t 타겟_이름` : 사용자 지정 빌드 타겟을 실행
- `cmake -h` : 도움말 표시

## Links
- More information: https://cmake.org/cmake/help/latest/manual/cmake.1.html
