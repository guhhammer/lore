# dnf4

RHEL 8/9 및 구버전 Fedora(pre-41)를 위한 패키지 관리자.

## Usage
- `sudo dnf4 up` : 설치된 패키지를 최신 사용 가능 버전으로 업그레이드
- `dnf4 se 키워드1 키워드2 ...` : 키워드로 패키지 검색
- `dnf4 if 패키지` : 패키지 상세 정보 표시
- `sudo dnf4 in 패키지1 패키지2 ...` : 새로운 패키지 설치 (`--assumeyes` 사용 시 모든 프롬프트 자동 승인)
- `sudo dnf4 rm 패키지1 패키지2 ...` : 패키지 제거
- `dnf4 ls --installed` : 설치된 패키지 목록 표시
- `dnf4 wp 명령어` : 특정 명령을 제공하는 패키지 검색
- `dnf4 hist` : 모든 과거 작업 기록 표시

## Links
- More information: https://wiki.archlinux.org/title/Pacman/Rosetta
