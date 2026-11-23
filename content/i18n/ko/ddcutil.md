# ddcutil

DDC/CI를 통해 연결된 디스플레이의 설정을 제어합니다. 이 명령은 `i2c-dev` 커널 모듈이 로드되어 있어야 합니다.

## Usage
- `ddcutil detect` : 호환 가능한 모든 디스플레이 나열
- `ddcutil --display 1 setvcp 10 50` : 디스플레이 1의 밝기(옵션 0x10)를 50%로 변경
- `ddcutil -d 1 setvcp 12 + 5` : 디스플레이 1의 대비(옵션 0x12)를 5% 증가
- `ddcutil -d 1 getvcp ALL` : 디스플레이 1의 설정 읽기

## Links
- More information: https://www.ddcutil.com/commands/
