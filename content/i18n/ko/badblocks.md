# badblocks

장치에서 불량 블록을 검색. 일부 불량 블록 사용은 파티션 테이블을 포함해 디스크의 모든 데이터를 지우는 등 돌이킬 수 없는 작업 유발 가능.

## Usage
- `sudo badblocks /dev/sdX` : 비파괴 읽기 전용 테스트를 사용하여 디스크에서 불량 블록을 검색
- `sudo badblocks -n /dev/sdX` : 비파괴(non-destructive) 읽기-쓰기 테스트를 통해 마운트 해제된 디스크에서 불량 블록을 검색
- `sudo badblocks -w /dev/sdX` : 파괴적인 쓰기(write) 테스트를 통해 마운트되지 않은 디스크에서 불량 블록을 검색
- `sudo badblocks -svw /dev/sdX` : 파괴적인 쓰기(write) 테스트를 사용하고 상세하게 진행(show verbose)
- `sudo badblocks -o 경로/대상/파일 -w /dev/sdX` : 파괴적인 모드에선, 발견된 블록을 파일로 출력(output)
- `sudo badblocks -w -b 4096 -c 65536 /dev/sdX` : 4K 블록(block) 크기 및 64K 블록 수(count)를 사용하여 속도가 향상된 파괴 모드를 사용

## Links
- More information: https://manned.org/badblocks
