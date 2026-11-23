# filefrag

특정 파일의 단편화 정도를 보고.

## Usage
- `filefrag 경로/대상/파일1 경로/대상/파일2 ...` : 하나 이상의 f파일에 대한 보고서 표시
- `filefrag -k 경로/대상/파일` : 1024 바이트 블록 크기를 사용하여 보고서 표시
- `filefrag -b1024|1K|1M|1G|... 경로/대상/파일` : 특정 블록 크기를 사용하여 보고서 표시
- `filefrag -s 경로/대상/파일1 경로/대상/파일2 ...` : 매핑 요청 전 f파일 동기화
- `filefrag -x 경로/대상/파일1 경로/대상/파일2 ...` : 확장 속성의 매핑 표시
- `filefrag -v 경로/대상/파일1 경로/대상/파일2 ...` : 자세한 정보를 포함한 보고서 표시

## Links
- More information: https://manned.org/filefrag
