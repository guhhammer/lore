# arduino-builder

아두이노 스케치 컴파일. 사용 중단 경고: 이 도구는 `arduino`로 인해 단계적으로 중단.

## Usage
- `arduino-builder -compile 경로/대상/sketch.ino` : 스케치를 작성
- `arduino-builder -debug-level 1..10` : 디버그 수준 지정 (기본값: 5)
- `arduino-builder -build-path 경로/대상/빌드_디렉터리` : 사용자 정의 빌드 디렉토리 지정
- `arduino-builder -build-options-file 경로/대상/build.options.json` : `-hardware`, `-tools` 등을 매번 수동으로 지정하는 대신, 빌드 옵션 파일을 사용
- `arduino-builder -verbose true` : 상세 모드 활성화

## Links
- More information: https://github.com/arduino/arduino-builder
