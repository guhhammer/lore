# strigger

Slurm 트리거 정보를 조회하거나 수정합니다. 트리거는 Slurm 클러스터에서 이벤트가 발생할 때 자동으로 실행되는 작업입니다.

## Usage
- `strigger --set --primary_database_failure|primary_slurmdbd_failure|primary_slurmctld_acct_buffer_full|primary_slurmctld_failure|... -p 경로/대상/실행파일` : 새로운 트리거 등록: 지정된 이벤트가 발생할 때 지정된 프로그램을 실행
- `strigger --set -j 작업_id -f -p "경로/대상/실행파일 인수1 인수2 ..."` : 지정된 작업이 종료될 때 지정된 프로그램을 실행
- `strigger --get` : 활성 트리거 보기
- `strigger --get -j 작업_id` : 지정된 작업과 관련된 활성 트리거 보기
- `strigger --clear 트리거_id` : 지정된 트리거 삭제

## Links
- More information: https://slurm.schedmd.com/strigger.html
