# sinfo

Slurm 노드 및 파티션 정보를 표시합니다.

## Usage
- `sinfo -s` : 클러스터의 빠른 요약 개요 표시
- `sinfo` : 클러스터 전체의 모든 파티션에 대한 자세한 상태 보기
- `sinfo -p 파티션_이름` : 특정 파티션에 대한 자세한 상태 보기
- `sinfo -t idle` : 유휴 노드 정보 보기
- `sinfo -d` : 죽은 노드 요약
- `sinfo -R` : 죽은 노드와 그 이유 나열

## Links
- More information: https://slurm.schedmd.com/sinfo.html
