# yarac

YARA 규칙 소스 파일을 더 빠르게 불러올 수 있도록 바이너리 형식으로 컴파일.

## Usage
- `yarac 경로/대상/규칙.yar 경로/대상/규칙.bin` : 지정한 규칙 파일 컴파일
- `yarac 경로/대상/규칙1.yar 경로/대상/규칙2.yar ... 경로/대상/규칙s.bin` : 여러 규칙 파일을 하나의 바이너리 파일로 컴파일
- `yarac -d 식별자=값 경로/대상/규칙.yar 경로/대상/규칙.bin` : 컴파일 시 외부 변수 정의
- `yarac -w 경로/대상/규칙.yar 경로/대상/규칙.bin` : 컴파일 중 경고 비활성화
- `yarac --fail-on-warnings 경로/대상/규칙.yar 경로/대상/규칙.bin` : 경고가 하나라도 발생하면 컴파일 실패로 처리 (`--no-warnings`와 함께 사용하지 않음)

## Links
- More information: https://manned.org/yarac
