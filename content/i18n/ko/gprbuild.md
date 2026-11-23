# gprbuild

Ada 및 기타 언어 (C/C++/Fortran)로 작성된 프로젝트를 위한 고급 빌드 도구.

## Usage
- `gprbuild` : 프로젝트를 빌드 (현재 디렉터리에 하나의 `*.gpr` 파일만 존재한다고 가정)
- `gprbuild -P 프로젝트_이름` : 특정 프로젝트(Project) 파일 빌드
- `gprclean` : 빌드 작업공간을 정리
- `gprinstall --prefix 경로/대상/설치/폴더` : 컴파일된 바이너리를 설치

## Links
- More information: https://docs.adacore.com/gprbuild-docs/html/gprbuild_ug.html
