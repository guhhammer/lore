# mpirun

Open MPI에서 직렬 및 병렬 작업 실행.

## Usage
- `mpirun 경로/대상/실행파일` : open MPI 프로그램 실행
- `mpirun -n n 경로/대상/실행파일` : `n`개의 병렬 프로세스로 Open MPI 프로그램 실행
- `mpirun -oversubscribe 경로/대상/실행파일` : 사용 가능한 물리 코어 수보다 많은 프로세스 실행 허용

## Links
- More information: https://docs.open-mpi.org/en/main/man-openmpi/man1/mpirun.1.html
