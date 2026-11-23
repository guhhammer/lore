# dpkg

Debian 패키지 관리자. `deb`와 같은 일부 하위 명령에는 자체 사용 설명서가 있습니다.

## Usage
- `sudo dpkg -i 경로/대상/파일.deb` : 패키지 설치
- `sudo dpkg -r 패키지` : 패키지 제거
- `dpkg -l 패턴` : 설치된 패키지 나열
- `dpkg -L 패키지` : 패키지의 내용 나열
- `dpkg -c 경로/대상/파일.deb` : 로컬 패키지 파일의 내용 나열
- `dpkg -S 경로/대상/파일` : 특정 파일이 어떤 패키지에 속해 있는지 확인
- `sudo dpkg -P 패키지` : 설치되었거나 이미 제거된 패키지 및 구성 파일 삭제

## Links
- More information: https://wiki.archlinux.org/title/Pacman/Rosetta
