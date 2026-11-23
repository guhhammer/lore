# sh5util

`sacct_gather_profile` 플러그인이 생성한 HDF5 파일 병합 도구.

## Usage
- `sh5util -j 작업_id|작업_id.단계_id` : 지정된 작업 또는 단계에 대해 각 할당된 노드에서 생성된 HDF5 파일 병합
- `sh5util -j 작업_id|작업_id.단계_id -E -i 경로/대상/파일.h5 -s Energy|Filesystem|Network|Task` : 병합된 작업 파일에서 하나 이상의 데이터 시리즈 추출
- `sh5util -j 작업_id|작업_id.단계_id -I -s Energy|Filesystem|Network|Task -d 데이터_항목` : 병합된 작업 파일에서 모든 노드의 하나의 데이터 항목 추출

## Links
- More information: https://slurm.schedmd.com/sh5util.html
