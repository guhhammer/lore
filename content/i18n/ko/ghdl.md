# ghdl

VHDL 언어용 오픈 소스 시뮬레이터.

## Usage
- `ghdl -a 파일이름.vhdl` : VHDL 소스 파일을 분석하고 개체 파일을 생성
- `ghdl -e 디자인` : 설계를 정교화 (여기서 `design`은 구성 단위, 엔터티 단위 또는 아키텍처 단위의 이름)
- `ghdl -r 디자인` : 정교한 디자인 실행
- `ghdl -r 디자인 --wave=output.ghw` : 정교한 설계를 실행하고 출력을 파형 파일로 덤프
- `ghdl -s 파일이름.vhdl` : VHDL 소스 파일의 구문을 확인
- `ghdl --help` : 도움말 표시

## Links
- More information: https://ghdl.github.io/ghdl/
