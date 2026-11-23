# iverilog

Verilog HDL (IEEE-1364) 코드를 전처리하고 시뮬레이션용 실행 파일로 컴파일 도구.

## Usage
- `iverilog 경로/대상/소스파일.v -o 경로/대상/실행파일` : 소스 파일을 실행 파일로 컴파일
- `iverilog 경로/대상/소스파일.v -Wall -o 경로/대상/실행파일` : 모든 경고를 표시하며 컴파일
- `iverilog -o 경로/대상/실행파일 -tvvp 경로/대상/소스파일.v` : VVP 런타임을 사용하여 컴파일 및 실행
- `iverilog 경로/대상/소스파일.v -o 경로/대상/실행파일 -I경로/대상/라이브러리_디렉터리` : 다른 경로의 Verilog 라이브러리를 사용하여 컴파일
- `iverilog -E 경로/대상/소스파일.v` : 컴파일 없이 Verilog 코드 전처리

## Links
- More information: https://manned.org/iverilog
