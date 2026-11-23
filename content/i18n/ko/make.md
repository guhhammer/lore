# make

Makefile에 작성된 대상에 대한 작업 실행기입니다. 주로 소스 코드에서 실행 파일의 컴파일을 제어하는 데 사용됩니다.

## Usage
- `make` : makefile에 지정된 첫 번째 대상(일반적으로 "all"이라는 이름)을 호출
- `make 대상` : 특정 대상을 호출
- `make -j 4 대상` : 특정 대상을 호출하여, 한 번에 4개의 작업을 병렬로 실행
- `make -f 경로/대상/파일` : 특정 Makefile을 사용
- `make -C 경로/대상/폴더` : 다른 디렉토리에서 make 실행
- `make -B 대상` : 소스 파일이 변경되지 않은 경우에도, 대상을 강제로 make 실행
- `make 대상 변수=새로운_값` : makefile에 정의된 변수를 재정의
- `make -e 대상` : 환경에 의해 Makefile에 정의된 변수를 재정의

## Links
- More information: https://www.gnu.org/software/make/manual/make.html
