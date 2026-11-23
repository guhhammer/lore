# ninja

빠른 빌드를 위해 설계된 빌드 시스템.

## Usage
- `ninja` : 현재 디렉토리에서 빌드
- `ninja -j 4` : 현재 디렉토리에서 빌드하고, 동시에 4개의 작업을 병렬로 실행
- `ninja -C 경로/대상/폴더` : 주어진 디렉토리에서 프로그램 빌드
- `ninja -t targets` : 대상 표시 (예: `install` 및 `uninstall`)
- `ninja -h` : 도움말 표시

## Links
- More information: https://ninja-build.org/manual.html
