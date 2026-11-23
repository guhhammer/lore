# sstat

실행 중인 작업에 대한 정보를 표시합니다.

## Usage
- `sstat -j 작업_id` : 쉼표로 구분된 작업 목록의 상태 정보를 표시
- `sstat -p -j 작업_id -o JobID,AveCPU,AveVMSize` : 쉼표로 구분된 작업 목록의 작업 ID, 평균 CPU 및 평균 가상 메모리 크기를 파이프로 구분하여 표시
- `sstat -e` : 사용 가능한 필드 목록 표시

## Links
- More information: https://slurm.schedmd.com/sstat.html
