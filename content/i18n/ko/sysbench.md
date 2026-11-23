# sysbench

시스템의 CPU, IO 및 메모리를 벤치마킹.

## Usage
- `sysbench cpu run` : 1개의 스레드로 10초 동안 CPU 벤치마크 실행
- `sysbench --threads=스레드_수 --time=초` : 여러 스레드로 지정된 시간 동안 CPU 벤치마크 실행
- `sysbench memory run` : 1개의 스레드로 10초 동안 메모리 벤치마크 실행
- `sysbench fileio prepare` : 파일 시스템 수준의 읽기 벤치마크 준비
- `sysbench --file-test-mode=rndrd|rndrw|rndwr|seqrd|seqrewr|seqwr fileio run` : 파일 시스템 수준의 벤치마크 실행

## Links
- More information: https://github.com/akopytov/sysbench#usage
