# xmake

Lua 기반의 크로스 플랫폼 C & C++ 빌드 유틸리티.

## Usage
- `xmake create -l c -P 프로젝트_이름` : hello World와 `xmake.lua`를 포함한 Xmake C 프로젝트 생성
- `xmake b r` : xmake 프로젝트 빌드 및 실행
- `xmake r 타겟_이름` : 컴파일된 Xmake 타겟을 직접 실행
- `xmake f -p macosx|linux|iphoneos|... -a x86_64|i386|arm64|... -m debug|release` : 프로젝트의 빌드 타겟 구성
- `xmake i -o 경로/대상/폴더` : 컴파일된 타겟을 디렉토리에 설치

## Links
- More information: https://github.com/xmake-io/xmake#command-line-interface-reference
