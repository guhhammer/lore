# cpupower

CPU 전력 및 조정 옵션 관련 도구.

## Usage
- `sudo cpupower -c all info` : CPU 목록 나열
- `sudo cpupower -c all info` : 모든 코어에 대한 정보 출력
- `sudo cpupower -c all frequency-set --governor powersave` : 모든 CPU를 절전 주파수 관리자로 설정
- `sudo cpupower -c 0 frequency-info -g | grep "analyzing\|governors"` : CPU 0의 사용 가능한 주파수 governor 출력
- `sudo cpupower -c 4 frequency-info -w -m` : CPU 4의 하드웨어 주파수를 사람이 읽기 쉬운 형식으로 출력

## Links
- More information: https://manned.org/cpupower
