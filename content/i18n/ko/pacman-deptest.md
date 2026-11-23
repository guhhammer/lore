# pacman-deptest

지정된 각 의존성을 확인하고 시스템에 현재 충족되지 않은 의존성 목록을 반환합니다.

## Usage
- `pacman -T 패키지1 패키지2 ...` : 설치되지 않은 의존성의 패키지 이름을 출력
- `pacman -T "bash>=5"` : 설치된 패키지가 주어진 최소 버전을 충족하는지 확인
- `pacman -T "bash>5"` : 패키지의 최신 버전이 설치되었는지 확인
- `pacman -Th` : 도움말 표시

## Links
- More information: https://manned.org/pacman.8
