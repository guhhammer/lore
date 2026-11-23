# yum

RHEL, Fedora, CentOS(이전 버전용)를 위한 패키지 관리 도구.

## Usage
- `yum install 패키지` : 새 패키지 설치
- `yum -y install 패키지` : 새 패키지를 설치하고 모든 질문에 대해 예로 가정 (업데이트에도 작동하며, 자동화된 업데이트에 유용)
- `yum provides 명령` : 특정 명령을 제공하는 패키지 찾기
- `yum remove 패키지` : 패키지 제거
- `yum check-update` : 설치된 패키지에 대한 사용 가능한 업데이트 표시
- `yum upgrade` : 설치된 패키지를 최신 버전으로 업그레이드

## Links
- More information: https://wiki.archlinux.org/title/Pacman/Rosetta
