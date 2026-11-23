# rcsmerge

RCS 리비전의 변경 사항을 작업 파일에 병합.

## Usage
- `rcsmerge -r리비전1 -r리비전2 경로/대상/파일` : 두 리비전 간 차이점을 작업 파일에 병합
- `rcsmerge -r브랜치_리비전 경로/대상/파일` : 브랜치 리비전의 변경 사항을 작업 파일에 병합
- `rcsmerge -q -r리비전1 -r리비전2 경로/대상/파일` : 메시지 표시 없이 병합 (진단 메시지 표시 하지 않음)
- `rcsmerge -p -r리비전1 -r리비전2 경로/대상/파일` : 작업 파일을 덮어쓰지 않고 병합 결과를 `stdout`으로 출력

## Links
- More information: https://manned.org/rcsmerge
