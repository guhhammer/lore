# pamixer

PulseAudio를 위한 간단한 커맨드라인 믹서.

## Usage
- `pamixer --list-sinks --list-sources` : 모든 싱크 및 소스를 해당 ID와 함께 나열
- `pamixer --set-volume 75` : 기본 싱크의 볼륨을 75%로 설정
- `pamixer --toggle-mute --sink id` : 기본이 아닌 싱크 음소거 전환
- `pamixer -i 5` : 기본 싱크의 볼륨을 5% 증가
- `pamixer -d 5 --source id` : 소스의 볼륨을 5% 감소
- `pamixer --set-volume 105 --allow-boost` : 100% 이상으로 볼륨을 증가, 감소 또는 설정하기 위해 부스트 허용 옵션 사용
- `pamixer -m` : 기본 싱크 음소거 (`--unmute`를 사용하여 음소거 해제 가능)

## Links
- More information: https://github.com/cdemoulins/pamixer#installation
