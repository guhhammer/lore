# dnf

RHEL, Fedora 및 CentOS를 위한 패키지 관리 도구(yum을 대체).

## Usage
- `sudo dnf upgrade` : 설치된 패키지를 최신 버전으로 업그레이드
- `dnf search 키워드1 키워드2 ...` : 키워드를 통해 패키지 검색
- `dnf info 패키지` : 패키지에 대한 세부 정보 표시
- `sudo dnf install 패키지1 패키지2 ...` : 새 패키지 설치 (`-y`를 사용하여 모든 프롬프트 자동 확인)
- `sudo dnf remove 패키지1 패키지2 ...` : 패키지 제거
- `dnf list --installed` : 설치된 패키지 나열
- `dnf provides 명령` : 특정 명령을 제공하는 패키지 찾기
- `dnf history` : 모든 과거 작업 보기

## Links
- More information: https://wiki.archlinux.org/title/Pacman/Rosetta
