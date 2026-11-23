# setserial

시리얼 포트 정보를 읽고 수정합니다.

## Usage
- `setserial -a /dev/cuaN` : 특정 시리얼 장치에 대한 모든 정보 출력
- `setserial -b 장치` : 특정 시리얼 장치의 구성 요약 출력 (부팅 과정에서 출력할 때 유용)
- `sudo setserial 장치 매개변수` : 장치에 특정 구성 매개변수 설정
- `setserial -g 장치1 장치2 ...` : 장치 목록의 구성 출력

## Links
- More information: https://manned.org/setserial
