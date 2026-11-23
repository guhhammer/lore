# runlim

Linux의 proc 파일 시스템을 사용하여 프로그램 및 자식 프로세스의 시간 및 메모리 사용량을 샘플링하고 제한합니다.

## Usage
- `runlim 명령어 명령어_인자` : 명령의 시간 및 메모리 사용량 출력
- `runlim --output-file=경로/대상/파일 명령어 명령어_인자` : 통계를 `stdout` 대신 파일에 기록
- `runlim --time-limit=숫자 명령어 명령어_인자` : 상한 시간(초 단위) 제한
- `runlim --real-time-limit=숫자 명령어 명령어_인자` : 실시간 상한(초 단위) 제한
- `runlim --space-limit=숫자 명령어 명령어_인자` : 공간 상한(MB 단위) 제한

## Links
- More information: https://fmv.jku.at/runlim/
