# bpkg

Bash 스크립트용 패키지 관리자.

## Usage
- `bpkg update` : 로컬 색인 업데이트
- `bpkg install --global 패키지` : 전역적으로 패키지를 설치
- `bpkg install 패키지` : 현재 디렉터리의 하위 디렉터리에 패키지를 설치
- `bpkg install 패키지@버전 -g` : 특정 버전의 패키지를 전역적으로 설치
- `bpkg show 패키지` : 특정 패키지에 대한 세부정보 표시
- `bpkg run 명령어 인자1 인자2 ...` : 선택적으로 인수를 지정하여, 명령을 실행

## Links
- More information: https://github.com/bpkg/bpkg
