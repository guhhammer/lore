# gamescope

게임 실행을 위한 micro-compositor.

## Usage
- `gamescope -- 프로그램` : gamescope를 사용해 프로그램 실행
- `gamescope -- %command%` : steam을 통해 gamescope로 게임 실행
- `gamescope -h 720 -H 1440 -S integer -- 명령어` : 720p 게임을 정수 스케일링으로 1440p까지 업스케일
- `gamescope -r 30 -- 명령어` : vsynce이 적용된 게임을 30 FPS로 제한
- `gamescope -e -- /usr/bin/steam -tenfoot` : steam을 Big Picture 모드로 실행하고 gamescope와 통합
- `gamescope -O HDMI-A-1,DP-3,... -- program` : 우선 사용할 디스플레이 지정
- `<Super f>` : 전체 화면 전환
- `gamescope --help` : 도움말 표시

## Links
- More information: https://github.com/ValveSoftware/gamescope#keyboard-shortcuts
