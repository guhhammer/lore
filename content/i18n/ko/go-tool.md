# go tool

Go 도구 또는 명령 실행. Go 명령을 독립 실행형 바이너리로 실행하여 주로 디버깅에 사용.

## Usage
- `go tool` : 사용 가능한 도구 나열
- `go tool link 경로/대상/main.o` : go link 도구 실행
- `go tool -n 명령 인수들` : 실행될 명령을 출력하지만 실제로 실행하지 않음 (`whereis`와 유사)
- `go tool 명령 --help` : 지정된 도구에 대한 문서 보기
- `go tool dist list` : 사용 가능한 모든 교차 컴파일 대상 나열

## Links
- More information: https://pkg.go.dev/cmd/go#hdr-Run_specified_go_tool
