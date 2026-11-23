# apt

Debian 기반 배포판을 위한 패키지 관리 도구. Ubuntu 16.04 이후 버전에서 대화형 사용 시 `apt-get`의 권장 대체 도구.

## Usage
- `sudo apt update` : 사용 가능한 패키지 및 버전 목록 업데이트 (다른 `apt` 명령어 실행 전 권장)
- `apt search 패키지` : 주어진 패키지 검색
- `apt show 패키지` : 패키지 정보 표시
- `sudo apt install 패키지` : 패키지 설치 또는 최신 버전으로 업데이트
- `sudo apt remove 패키지` : 패키지 제거 (`purge`를 사용하면 설정 파일도 함께 제거)
- `sudo apt upgrade` : 모든 설치된 패키지를 최신 버전으로 업그레이드
- `apt list` : 모든 패키지 나열
- `apt list -i` : 설치된 패키지 나열

## Links
- More information: https://wiki.archlinux.org/title/Pacman/Rosetta
